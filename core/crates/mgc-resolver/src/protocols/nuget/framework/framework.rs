//! NuGet target-framework monikers and nearest-compatible group selection.
//! Moniker target framework NuGet và chọn group tương thích gần nhất.

use crate::protocols::nuget::tag_attr;
use mgc_types::Version;

/// Read the `targetFramework` attribute of a `<group>` block.
/// Đọc attribute `targetFramework` của khối `<group>`.
pub(in crate::protocols::nuget) fn group_tfm(group_xml: &str) -> Option<String> {
    tag_attr(group_xml, "targetFramework")
}

/// Framework family for TFM compatibility ranking.
/// (Họ framework để xếp hạng tương thích TFM.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TfmFamily {
    /// Modern .NET (`net5.0`–`net9.0`, plain `net8.0`).
    Net,
    /// .NET Framework (`net45`, `net472`, `net48`, long form).
    NetFx,
    /// .NET Core (`netcoreapp3.1`).
    NetCore,
    /// .NET Standard (`netstandard2.0`).
    NetStandard,
}

/// Parse a short (or common long) TFM moniker into (family, version).
/// Platform suffixes (`net6.0-android`) are stripped to the base TFM.
/// (Parse moniker TFM thành (họ, version).)
fn parse_tfm(moniker: &str) -> Option<(TfmFamily, Version)> {
    let base = moniker
        .split(['-', '+'])
        .next()
        .unwrap_or(moniker)
        .trim()
        .to_ascii_lowercase();
    let base = base
        .strip_prefix(".netframework")
        .map(|v| format!("net{v}"))
        .unwrap_or(base);
    let base = base
        .strip_prefix(".netstandard")
        .map(|v| format!("netstandard{v}"))
        .unwrap_or(base);
    let base = base
        .strip_prefix(".netcoreapp")
        .map(|v| format!("netcoreapp{v}"))
        .unwrap_or(base);
    if let Some(v) = base.strip_prefix("netstandard") {
        return Version::parse(&norm_tfm_version(v))
            .ok()
            .map(|ver| (TfmFamily::NetStandard, ver));
    }
    if let Some(v) = base.strip_prefix("netcoreapp") {
        return Version::parse(&norm_tfm_version(v))
            .ok()
            .map(|ver| (TfmFamily::NetCore, ver));
    }
    if let Some(v) = base.strip_prefix("net") {
        // `net48`/`net472` (no dots) are .NET Framework; dotted `net8.0`
        // is modern .NET. (`net48` là Framework; `net8.0` là .NET mới.)
        if v.contains('.') {
            return Version::parse(&norm_tfm_version(v))
                .ok()
                .map(|ver| (TfmFamily::Net, ver));
        }
        let dotted = match v.len() {
            2 => format!("{}.{}", &v[0..1], &v[1..2]),
            3 => format!("{}.{}", &v[0..1], &v[1..3]),
            _ => return None,
        };
        return Version::parse(&dotted)
            .ok()
            .map(|ver| (TfmFamily::NetFx, ver));
    }
    None
}

/// Normalize partial versions (`8` → `8.0.0`, `4.8` → `4.8.0`) for parsing.
/// (Chuẩn hóa version thiếu phần.)
fn norm_tfm_version(v: &str) -> String {
    let parts: Vec<&str> = v.split('.').collect();
    match parts.len() {
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => v.to_string(),
    }
}

/// Compatibility rank of a package `candidate` group for a `consumer` TFM
/// (higher = better; `None` = incompatible). Rules mirror NuGet's own
/// nearest-compatible selection, bounded to the four families above:
/// exact match wins; same-family lower versions rank by version;
/// netstandard candidates rank when within the consumer's supported
/// ceiling (net5+/netcoreapp3.0+: 2.1; netcoreapp2.x/net472+: 2.0).
/// (Xếp hạng tương thích group theo TFM consumer.)
pub(in crate::protocols::nuget) fn tfm_compat_rank(consumer: &str, candidate: &str) -> Option<u64> {
    let (cfam, cver) = parse_tfm(consumer)?;
    let (kfam, kver) = parse_tfm(candidate)?;
    if cfam == kfam {
        if kver == cver {
            return Some(u64::MAX);
        }
        if kver < cver {
            return Some(1_000_000 + kver.major * 10_000 + kver.minor * 100 + kver.patch);
        }
        return None;
    }
    if kfam == TfmFamily::NetStandard {
        let ceiling: Version = match cfam {
            TfmFamily::Net => Version::parse("2.1.0").ok()?,
            TfmFamily::NetCore if cver >= Version::parse("3.0.0").ok()? => {
                Version::parse("2.1.0").ok()?
            }
            TfmFamily::NetCore | TfmFamily::NetFx => Version::parse("2.0.0").ok()?,
            TfmFamily::NetStandard => return None,
        };
        if kver <= ceiling {
            return Some(500_000 + kver.major * 10_000 + kver.minor * 100 + kver.patch);
        }
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
