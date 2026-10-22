//! Public PackageReference model and .csproj reference extraction.
//! Model PackageReference công khai và trích xuất reference trong .csproj.

use crate::protocols::nuget::{tag_attr, tag_blocks};

/// One `<PackageReference Include="…" Version="…"/>` reference.
/// Một tham chiếu `<PackageReference Include="…" Version="…"/>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageReference {
    pub id: String,
    pub version: Option<String>,
}

/// Parse all `<PackageReference>` entries from a .csproj (text scan, no
/// XML crate — attributes carry the data). Entries without a Version
/// attribute or with a floating `*` version are returned in `unresolved`
/// for honest skip+marker handling by the caller.
/// Parse mọi `<PackageReference>` từ .csproj (quét text, không XML crate —
/// dữ liệu nằm trong attribute). Entry thiếu attribute Version hoặc version
/// float `*` trả về trong `unresolved` để caller skip+marker trung thực.
pub fn parse_package_references(csproj: &str) -> (Vec<PackageReference>, Vec<String>) {
    let mut resolved = Vec::new();
    let mut unresolved = Vec::new();
    for tag in tag_blocks(csproj, "PackageReference") {
        let Some(id) = tag_attr(tag, "Include") else {
            continue;
        };
        match tag_attr(tag, "Version") {
            Some(v) if !v.is_empty() && v != "*" && !v.ends_with('*') => {
                resolved.push(PackageReference {
                    id,
                    version: Some(v),
                });
            }
            _ => unresolved.push(id),
        }
    }
    (resolved, unresolved)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
