use mgc_types::Version;

/// Sparse-index prefix rule for a crate name.
/// Luật prefix sparse-index cho tên crate.
pub(in crate::protocols) fn sparse_prefix(name: &str) -> String {
    match name.len() {
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => format!("3/{}", &name[..1]),
        _ => format!("{}/{}", &name[..2], &name[2..4]),
    }
}

/// Use Cargo-compatible SemVer parsing rather than a partial hand-written matcher.
/// Dùng parser SemVer cùng quy tắc Cargo.
pub(in crate::protocols) fn cargo_matches(range: &str, version: &Version) -> bool {
    let Ok(candidate) = semver::Version::parse(&version.to_string()) else {
        return false;
    };
    semver::VersionReq::parse(range.trim()).is_ok_and(|requirement| requirement.matches(&candidate))
}
