//! Map MGC ecosystem labels into package URLs and component references.
//! Map nhãn ecosystem MagiCore thành package URL và component reference.

use mgc_types::adapter::Vulnerability;

/// Mapping table from package ecosystem tag to purl type prefix
/// (RULE §12: one central table, not scattered literals). Covers every
/// ecosystem tag the scanners actually stamp (P1: Maven/NuGet/Go/Pub
/// findings used to fall through to `generic`, useless for downstream
/// tooling).
/// Bảng map tag ecosystem → tiền tố purl (RULE §12: một bảng tập trung,
/// không rải literal). Phủ mọi tag scanner thật sự gắn.
const PURL_TYPES: &[(&str, &str)] = &[
    ("rust", "cargo"),
    ("python", "pypi"),
    ("web/javascript", "npm"),
    ("kotlin", "maven"),
    ("java", "maven"),
    ("Maven", "maven"),
    ("dotnet", "nuget"),
    ("NuGet", "nuget"),
    ("go", "golang"),
    ("Go", "golang"),
    ("Pub", "pub"),
    ("flutter", "pub"),
    ("SwiftURL", "swift"),
    ("swift", "swift"),
    ("model-artifact", "generic"),
];

/// Default purl type when the ecosystem tag is missing/unknown.
/// Purl type mặc định khi thiếu/không biết tag ecosystem.
const PURL_TYPE_DEFAULT: &str = "generic";

/// Ecosystem tag → purl type prefix ("rust" → "cargo").
/// Tag ecosystem → tiền tố purl ("rust" → "cargo").
pub(in crate::output::cyclonedx) fn purl_type_for(ecosystem: Option<&str>) -> &'static str {
    ecosystem
        .and_then(|tag| PURL_TYPES.iter().find(|(k, _)| *k == tag).map(|(_, v)| *v))
        .unwrap_or(PURL_TYPE_DEFAULT)
}

/// Component bom-ref: `purl:<type>/<name>@<version>` — stable within one
/// report render, unique per affected package. Maven `group:artifact`
/// names become `group/artifact` per the purl spec.
/// Bom-ref component: `purl:<type>/<name>@<version>` — ổn định trong một
/// lần render, duy nhất theo từng package dính.
pub(in crate::output::cyclonedx) fn component_bom_ref(v: &Vulnerability) -> String {
    format!(
        "purl:{}/{}@{}",
        purl_type_for(v.ecosystem.as_deref()),
        purl_name(v),
        v.package.version()
    )
}

/// Human/package name as it appears in components: Maven coordinates
/// translate `:` to `/`; everything else passes through untouched.
/// Tên package trong component: tọa độ Maven đổi `:` thành `/`.
pub(in crate::output::cyclonedx) fn purl_name(v: &Vulnerability) -> String {
    let name = v.package.name().as_str().to_string();
    match purl_type_for(v.ecosystem.as_deref()) {
        "maven" => name.replacen(':', "/", 1),
        _ => name,
    }
}
