//! Resolved dependency graph API and graph-level helpers. — API đồ thị dependency đã resolve và helper cấp đồ thị.

use super::super::ResolvedPackage;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolvedGraph {
    pub packages: Vec<ResolvedPackage>,
}

impl ResolvedGraph {
    pub fn empty() -> Self {
        Self { packages: vec![] }
    }

    pub fn len(&self) -> usize {
        self.packages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }
}
