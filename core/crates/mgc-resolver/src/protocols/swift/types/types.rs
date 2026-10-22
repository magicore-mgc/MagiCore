use crate::protocols::swift::git_source::{git_url_to_name, identity_to_dep_name};

/// A dependency declared by a Package.swift / dump-package document.
/// Một dependency được khai báo trong Package.swift / dump-package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwiftDep {
    /// `.package(id: "scope.name", …)` — registry-sourced.
    /// `.package(id: "scope.name", …)` — nguồn registry.
    Registry {
        identity: String,
        requirement: String,
    },
    /// `.package(url: "…", …)` — git-only.
    /// `.package(url: "…", …)` — chỉ git.
    Git { url: String, requirement: String },
}

impl SwiftDep {
    /// Graph edge name: registry identity `scope.name` → `scope/name`;
    /// git URL → scheme-less host path (`https://host/x/y.git` →
    /// `host/x/y`, file:// URLs ride as-is — PackageName accepts `:`).
    /// Tên cạnh graph: registry identity `scope.name` → `scope/name`;
    /// git URL → host path không scheme (`https://host/x/y.git` →
    /// `host/x/y`, URL file:// giữ nguyên — PackageName chấp nhận `:`).
    pub fn dep_name(&self) -> String {
        match self {
            SwiftDep::Registry { identity, .. } => identity_to_dep_name(identity),
            SwiftDep::Git { url, .. } => git_url_to_name(url),
        }
    }
}

/// One pin imported from (or exported to) a Package.resolved file.
/// Một pin nhập từ (hoặc xuất ra) file Package.resolved.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SwiftResolvedPin {
    pub identity: String,
    pub version: Option<String>,
    pub revision: Option<String>,
    pub branch: Option<String>,
    pub location: Option<String>,
}

impl SwiftDep {
    /// Range string carried on the graph edge.
    /// Chuỗi range mang trên cạnh graph.
    pub fn requirement_text(&self) -> String {
        match self {
            SwiftDep::Registry { requirement, .. } | SwiftDep::Git { requirement, .. } => {
                requirement.clone()
            }
        }
    }
}
