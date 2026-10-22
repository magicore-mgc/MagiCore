//! Owns the reserved MSBuild PackageReference projection; no freshness gate is added. — Sở hữu projection PackageReference MSBuild đang để dành; không thêm freshness gate.

/// Extract `PackageReference` ids from MSBuild project files.
/// (Trích id `PackageReference` từ file project MSBuild.)
///
/// Reserved: same lock-shape caveat as [`declared_maven_deps`].
/// (Dành sẵn: cùng lưu ý dạng tên lock như [`declared_maven_deps`].)
#[allow(dead_code)]
pub fn declared_nuget_deps(csproj_xml: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = csproj_xml;
    while let Some(start) = rest.find("PackageReference") {
        rest = &rest[start + "PackageReference".len()..];
        let end = rest.find('>').unwrap_or(rest.len());
        let tag = &rest[..end];
        if let Some(include) = tag
            .split("Include=\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
        {
            let include = include.trim();
            if !include.is_empty() {
                names.push(include.to_string());
            }
        }
        rest = &rest[end.min(rest.len())..];
    }
    names.sort();
    names.dedup();
    names
}
