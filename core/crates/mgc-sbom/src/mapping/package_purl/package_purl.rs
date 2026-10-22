//! Ecosystem-specific package URL generation — Tạo Package URL theo từng hệ sinh thái

use crate::mapping::path_encoding::{percent_decode_path_segment, percent_encode_path, purl_path};
use mgc_lockfile::EcosystemTag;

pub(crate) fn package_purl(ecosystem: EcosystemTag, name: &str, version: &str) -> Option<String> {
    if name.trim().is_empty() || version.trim().is_empty() {
        return None;
    }
    let (kind, path) = match ecosystem {
        EcosystemTag::Web => ("npm", purl_path(name)),
        EcosystemTag::Rust => ("cargo", purl_path(name)),
        EcosystemTag::Python => ("pypi", purl_path(&normalize_pypi_name(name))),
        EcosystemTag::Dart => ("pub", purl_path(&normalize_pub_name(name))),
        EcosystemTag::Go => {
            if name.bytes().any(|byte| byte.is_ascii_uppercase()) {
                return None;
            }
            ("golang", purl_path(name))
        }
        EcosystemTag::Maven => {
            let (group, artifact) = name.split_once(':')?;
            if group.is_empty() || artifact.is_empty() {
                return None;
            }
            (
                "maven",
                format!("{}/{}", purl_path(group), purl_path(artifact)),
            )
        }
        EcosystemTag::NuGet => ("nuget", purl_path(name)),
        EcosystemTag::Swift => ("swift", purl_path(&swift_purl_path(name)?)),
        EcosystemTag::CocoaPods => ("cocoapods", purl_path(name)),
        EcosystemTag::Unity
        | EcosystemTag::Unreal
        | EcosystemTag::Iot
        | EcosystemTag::Model
        | EcosystemTag::CloudModule
        | EcosystemTag::Other => return None,
    };
    Some(format!(
        "pkg:{kind}/{path}@{}",
        percent_encode_path(version)
    ))
}

fn normalize_pypi_name(name: &str) -> String {
    let mut output = String::with_capacity(name.len());
    let mut previous_separator = false;
    for character in name.chars() {
        if matches!(character, '-' | '_' | '.') {
            if !previous_separator {
                output.push('-');
            }
            previous_separator = true;
        } else {
            output.extend(character.to_lowercase());
            previous_separator = false;
        }
    }
    output
}

fn normalize_pub_name(name: &str) -> String {
    name.chars()
        .map(|character| match character {
            'A'..='Z' => character.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' | '_' => character,
            _ => '_',
        })
        .collect()
}

pub(crate) fn swift_purl_path(name: &str) -> Option<String> {
    let mut path = name.trim();
    if let Ok(url) = url::Url::parse(path) {
        if !matches!(url.scheme(), "https" | "http") {
            return None;
        }
        let host = url.host_str()?;
        let mut segments = vec![host.to_string()];
        segments.extend(
            url.path_segments()?
                .filter(|segment| !segment.is_empty())
                .map(percent_decode_path_segment),
        );
        if segments
            .last()
            .is_some_and(|segment| segment.ends_with(".git"))
        {
            let last = segments.last_mut()?;
            last.truncate(last.len() - 4);
        }
        return (segments.len() >= 3).then(|| segments.join("/"));
    }
    if path.contains("://") {
        return None;
    }
    path = path.split(['?', '#']).next()?;
    path = path.strip_suffix(".git").unwrap_or(path);
    let segments: Vec<_> = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    (segments.len() >= 3).then(|| segments.join("/"))
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
