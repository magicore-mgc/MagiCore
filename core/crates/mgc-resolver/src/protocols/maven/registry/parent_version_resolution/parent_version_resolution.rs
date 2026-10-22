//! Resolve Maven dependency versions and merge lazy parent-POM knowledge.
//! Resolve version dependency Maven và merge dữ liệu POM cha theo nhu cầu.
use super::super::super::pom_coordinates::{parse_parent_coords, substitute_properties};
use super::super::super::pom_properties::{collect_managed_versions, collect_pom_properties};
use crate::protocols::maven::registry::registry::MavenProtocol;
use mgc_types::{MgError, MgResult};

impl MavenProtocol {
    /// Materialize a resolved artifact into the local Maven repository
    /// layout `{m2_root}/repository/{gpath}/{artifact}/{version}/` (jar +
    /// pom + published checksums) — readable by `mvn -o`.
    /// Materialize artifact đã resolve vào layout local Maven repository
    /// Resolve one dependency version: literal → properties →
    /// dependencyManagement → lazy parent chain → fail-closed error.
    /// Parent POMs fetch ONLY when needed (an unneeded unreachable parent
    /// must not break resolution); at most 3 ancestors, cycle-guarded.
    /// (Resolve version một dependency: literal → properties →
    /// dependencyManagement → chuỗi parent lười → lỗi fail-closed. POM cha
    /// chỉ fetch khi cần; tối đa 3 đời, chặn chu trình.)
    #[allow(clippy::too_many_arguments)]
    pub(in crate::protocols::maven) async fn maven_dep_version(
        &self,
        pom: &str,
        group: &str,
        artifact: &str,
        version: Option<&str>,
        props: &mut std::collections::HashMap<String, String>,
        managed: &mut std::collections::HashMap<(String, String), String>,
        visited_parents: &mut std::collections::HashSet<String>,
    ) -> MgResult<String> {
        // Merge parent knowledge lazily until this dep resolves or the
        // chain is exhausted (at most 3 ancestor fetches, cycle-guarded;
        // `current` walks up the chain so grandparents resolve too).
        // (Merge tri thức cha lười tới khi dep này resolve xong hoặc hết
        // chuỗi.)
        //
        // REVIEW: an empty `<version></version>` is a missing version, not
        // an empty-string edge — it takes the managed/parent path exactly
        // like an absent version (real Maven semantics).
        let version = version.filter(|v| !v.trim().is_empty());
        let mut current = pom.to_string();
        for _ in 0..4 {
            if let Some(raw) = version {
                if !raw.contains("${") {
                    return Ok(raw.to_string());
                }
                if let Some(resolved) = substitute_properties(raw, props) {
                    return Ok(resolved);
                }
            } else if let Some(pinned) = managed.get(&(group.to_string(), artifact.to_string())) {
                return Ok(pinned.clone());
            }
            match self
                .merge_parent_data(&current, props, managed, visited_parents)
                .await?
            {
                Some(parent_pom) => current = parent_pom,
                None => break,
            }
        }
        match version {
            Some(raw) => Err(MgError::Other(format!(
                "unresolvable version '{raw}' for {group}:{artifact} — declare a literal version, a <properties> entry, or a resolvable parent POM (fail-closed, no silent drop)"
            ))),
            None => Err(MgError::Other(format!(
                "no version for {group}:{artifact} and no dependencyManagement entry (checked parent chain) — declare one (fail-closed, no silent drop)"
            ))),
        }
    }

    /// Merge ONE ancestor level (properties + managed versions, child
    /// wins): returns the parent POM text when a new ancestor was merged,
    /// None when the chain ends here. Unreachable parents fail (needed
    /// data must not come from nowhere); a missing/unparseable `<parent>`
    /// block simply ends the chain.
    /// (Merge MỘT tầng tổ tiên: trả text POM cha khi merge được tầng mới,
    /// None khi hết chuỗi.)
    pub(in crate::protocols::maven) async fn merge_parent_data(
        &self,
        pom: &str,
        props: &mut std::collections::HashMap<String, String>,
        managed: &mut std::collections::HashMap<(String, String), String>,
        visited_parents: &mut std::collections::HashSet<String>,
    ) -> MgResult<Option<String>> {
        let Some((group, artifact, version)) = parse_parent_coords(pom)? else {
            return Ok(None);
        };
        let coordinate = format!("{group}:{artifact}:{version}");
        if visited_parents.contains(&coordinate) || visited_parents.len() >= 3 {
            return Ok(None);
        }
        visited_parents.insert(coordinate);
        let url = self.repo_url_for(
            &group,
            &artifact,
            &version,
            &format!("{artifact}-{version}.pom"),
        );
        let (status, parent_pom) = self.get_text(&url).await?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {url} returned {status} — parent POM is required to resolve versions (fail-closed)"
            )));
        }
        for (key, value) in collect_pom_properties(&parent_pom)? {
            props.entry(key).or_insert(value);
        }
        for (key, value) in collect_managed_versions(&parent_pom)? {
            managed.entry(key).or_insert(value);
        }
        Ok(Some(parent_pom))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
