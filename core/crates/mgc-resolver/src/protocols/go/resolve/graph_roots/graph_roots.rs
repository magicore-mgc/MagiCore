//! Traverse Go requirements and retain the highest selected version per module.
//! Duyệt requirement Go và giữ version cao nhất đã chọn của mỗi module.

use crate::protocols::go::{GoModProtocol, parse_go_version};
use crate::protocols::{RegistryProtocol, ResolvedEntry};
use mgc_types::MgResult;

impl GoModProtocol {
    /// Resolve all reachable roots using Go's minimum version selection.
    /// Resolve mọi root truy cập được theo minimum version selection của Go.
    pub(in crate::protocols::go::resolve) async fn resolve_mvs_roots(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let mut queue = std::collections::VecDeque::from(roots.to_vec());
        let mut visited = std::collections::BTreeSet::new();
        let mut selected = std::collections::BTreeMap::<String, ResolvedEntry>::new();

        while let Some((name, requirement)) = queue.pop_front() {
            if !visited.insert((name.clone(), requirement.clone())) {
                continue;
            }
            let entry = self.resolve(&name, &requirement).await?;
            let version = parse_go_version(&entry.version)?;
            let replace = match selected.get(&entry.name) {
                Some(previous) => {
                    let previous_version = parse_go_version(&previous.version)?;
                    version > previous_version
                }
                None => true,
            };

            // Every reachable requirement contributes its edges to the Go module graph.
            // Mọi requirement truy cập được đều bổ sung cạnh vào graph module Go.
            queue.extend(entry.deps.iter().cloned());
            if replace {
                selected.insert(entry.name.clone(), entry);
            }
        }

        Ok(selected.into_values().collect())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
