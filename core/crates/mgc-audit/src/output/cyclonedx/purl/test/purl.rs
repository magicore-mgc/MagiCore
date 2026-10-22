//! Verify ecosystem mapping and Maven component reference rules.
//! Kiểm tra mapping ecosystem và quy tắc reference component Maven.

use super::*;
use mgc_types::adapter::{FindingClass, Vulnerability, VulnerabilitySeverity};
use mgc_types::{PackageId, PackageName, Version};

fn row(ecosystem: Option<&str>, name: &str) -> Vulnerability {
    Vulnerability {
        package: PackageId::new(
            PackageName::new(name.to_string()).unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        title: "test finding".to_string(),
        severity: "high".to_string(),
        cve: "CVE-TEST".to_string(),
        severity_level: VulnerabilitySeverity::High,
        patched_versions: None,
        url: None,
        scanner: None,
        ecosystem: ecosystem.map(str::to_string),
        evidence_at: None,
        finding_class: FindingClass::Vulnerability,
    }
}

#[test]
fn purl_type_table_keeps_all_known_tags_and_generic_fallback() {
    for (tag, expected) in [
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
    ] {
        assert_eq!(purl_type_for(Some(tag)), expected, "{tag}");
    }
    assert_eq!(purl_type_for(None), "generic");
    assert_eq!(purl_type_for(Some("future-ecosystem")), "generic");
}

#[test]
fn maven_coordinates_use_slash_in_name_and_component_reference() {
    let vulnerability = row(Some("Maven"), "com.example:library:classifier");

    assert_eq!(purl_name(&vulnerability), "com.example/library:classifier");
    assert_eq!(
        component_bom_ref(&vulnerability),
        "purl:maven/com.example/library:classifier@1.0.0"
    );
}
