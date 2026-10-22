//! Fetch and select the package dependency graph from NuGet nuspec TFM groups.
//! Tải và chọn dependency graph từ các group TFM trong nuspec NuGet.

use crate::protocols::nuget::{NuGetProtocol, group_tfm, tag_attr, tag_blocks, tfm_compat_rank};
use mgc_types::{MgError, MgResult};

type DependencyList = Vec<(String, String)>;
type DependencyGroup = (Option<String>, DependencyList);
type RankedDependencyGroup = (u64, DependencyList, String);

/// Selected dependencies and their resolver markers, kept together by this owner.
/// Dependency được chọn và marker resolver do owner này quản lý cùng nhau.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::protocols::nuget::resolve) struct ResolvedDependencyGraph {
    pub(in crate::protocols::nuget::resolve) dependencies: DependencyList,
    pub(in crate::protocols::nuget::resolve) markers: Vec<String>,
}

/// Fetch the required flat-container nuspec and resolve its dependency groups.
/// Tải nuspec flat-container bắt buộc rồi resolve các dependency group.
pub(in crate::protocols::nuget::resolve) async fn resolve_dependencies(
    protocol: &NuGetProtocol,
    id: &str,
    version: &str,
    consumer_tfm: Option<&str>,
) -> MgResult<ResolvedDependencyGraph> {
    let nuspec_url = protocol.nuspec_url(id, version);
    let (status, nuspec) = protocol.get_text(&nuspec_url).await?;
    if !(200..300).contains(&status) {
        return Err(MgError::Network(format!(
            "GET {nuspec_url} returned {status} — the nuspec is required to build the graph \
             (fail-closed)"
        )));
    }
    resolve_nuspec_dependencies(id, version, &nuspec, consumer_tfm)
}

/// Parse each nuspec TFM group before selecting or combining dependency sets.
/// Parse từng group TFM trước khi chọn hoặc kết hợp các tập dependency.
fn resolve_nuspec_dependencies(
    id: &str,
    version: &str,
    nuspec: &str,
    consumer_tfm: Option<&str>,
) -> MgResult<ResolvedDependencyGraph> {
    // Do not silently combine framework groups with different dependency sets.
    // Không âm thầm gộp group framework có tập dependency khác nhau.
    let mut markers = Vec::new();
    let mut grouped: Vec<DependencyGroup> = Vec::new();
    for group in tag_blocks(nuspec, "group") {
        let tfm = group_tfm(group);
        if let Some(tfm) = tfm.as_deref() {
            markers.push(format!("group-tfm:{tfm}"));
        }
        let mut group_dependencies = Vec::new();
        for dependency_xml in tag_blocks(group, "dependency") {
            let attribute = |name: &str| tag_attr(dependency_xml, name);
            let Some(dependency_id) = attribute("id") else {
                continue;
            };
            match attribute("version") {
                Some(dependency_version) if !dependency_version.is_empty() => {
                    group_dependencies.push((dependency_id, dependency_version));
                }
                // Missing versions are skipped but remain visible in resolution metadata.
                // Thiếu version thì bỏ dependency, nhưng vẫn ghi nhận trong metadata resolve.
                _ => markers.push(format!("unresolved-version:{dependency_id}")),
            }
        }
        grouped.push((tfm, group_dependencies));
    }

    let mut distinct_tfms: Vec<&str> = grouped
        .iter()
        .filter_map(|(tfm, _)| tfm.as_deref())
        .collect();
    distinct_tfms.sort_unstable();
    distinct_tfms.dedup();

    let mut dependencies = Vec::new();
    if distinct_tfms.len() > 1 {
        let mut distinct_dependency_sets: Vec<Vec<(String, String)>> = grouped
            .iter()
            .filter(|(tfm, _)| tfm.is_some())
            .map(|(_, group_dependencies)| {
                let mut sorted = group_dependencies.clone();
                sorted.sort();
                sorted
            })
            .collect();
        distinct_dependency_sets.sort();
        distinct_dependency_sets.dedup();

        if distinct_dependency_sets.len() > 1 {
            if let Some(consumer) = consumer_tfm {
                let mut best: Option<RankedDependencyGroup> = None;
                for (tfm, group_dependencies) in &grouped {
                    let Some(candidate_tfm) = tfm.as_deref() else {
                        continue;
                    };
                    if let Some(rank) = tfm_compat_rank(consumer, candidate_tfm)
                        && best
                            .as_ref()
                            .is_none_or(|(current_rank, _, _)| rank > *current_rank)
                    {
                        best = Some((rank, group_dependencies.clone(), candidate_tfm.to_string()));
                    }
                }
                match best {
                    Some((_, selected, selected_tfm)) => {
                        markers.push(format!("tfm-selected:{selected_tfm}"));
                        dependencies.extend(selected);
                    }
                    None => {
                        return Err(MgError::Other(format!(
                            "multi-target package {id} {version} has divergent dependency sets \
                             across frameworks ({}) — none compatible with consumer '{consumer}' \
                             (fail-closed)",
                            distinct_tfms.join(", ")
                        )));
                    }
                }
            } else {
                return Err(MgError::Other(format!(
                    "multi-target package {id} {version} has divergent dependency sets across \
                     frameworks ({}) — target-framework selection is required \
                     (fail-closed, no silent merge)",
                    distinct_tfms.join(", ")
                )));
            }
        } else {
            markers.push(format!("multi-tfm-identical:{}", distinct_tfms.join(",")));
        }

        // A selected TFM already contributed one group; only identical sets merge.
        // Đã chọn TFM thì đã thêm một group; chỉ tập giống nhau mới được gộp.
        if !markers
            .iter()
            .any(|marker| marker.starts_with("tfm-selected:"))
        {
            for (_, group_dependencies) in &grouped {
                dependencies.extend(group_dependencies.iter().cloned());
            }
        }
    } else {
        for (_, group_dependencies) in &grouped {
            dependencies.extend(group_dependencies.iter().cloned());
        }
    }

    Ok(ResolvedDependencyGraph {
        dependencies,
        markers,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
