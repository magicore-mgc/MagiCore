//! Verify current ecosystem parsing, formatting and value traits.
//! Kiểm tra parser, formatter và trait value ecosystem hiện tại.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::{Ecosystem, NamespaceRole};

#[test]
fn from_str_all_variants() {
    assert_eq!(Ecosystem::from_str("web"), Some(Ecosystem::Web));
    assert_eq!(Ecosystem::from_str("game"), Some(Ecosystem::Game));
    assert_eq!(Ecosystem::from_str("ai"), Some(Ecosystem::Ai));
    assert_eq!(Ecosystem::from_str("clo"), Some(Ecosystem::Cloud));
    assert_eq!(Ecosystem::from_str("cicd"), Some(Ecosystem::Cicd));
    assert_eq!(Ecosystem::from_str("iot"), Some(Ecosystem::Iot));
    assert_eq!(Ecosystem::from_str("app"), Some(Ecosystem::App));
    assert_eq!(Ecosystem::from_str("lib"), Some(Ecosystem::Lib));
    assert_eq!(Ecosystem::from_str("hardware"), Some(Ecosystem::Hardware));
}

#[test]
fn from_str_cloud_alias() {
    assert_eq!(Ecosystem::from_str("cloud"), Some(Ecosystem::Cloud));
}

#[test]
fn from_str_unknown_returns_none() {
    assert_eq!(Ecosystem::from_str("unknown"), None);
    assert_eq!(Ecosystem::from_str(""), None);
}

#[test]
fn as_str_all_variants() {
    assert_eq!(Ecosystem::Web.as_str(), "web");
    assert_eq!(Ecosystem::Game.as_str(), "game");
    assert_eq!(Ecosystem::Ai.as_str(), "ai");
    assert_eq!(Ecosystem::Cloud.as_str(), "clo");
    assert_eq!(Ecosystem::Cicd.as_str(), "cicd");
    assert_eq!(Ecosystem::Iot.as_str(), "iot");
    assert_eq!(Ecosystem::App.as_str(), "app");
    assert_eq!(Ecosystem::Lib.as_str(), "lib");
    assert_eq!(Ecosystem::Hardware.as_str(), "hardware");
}

#[test]
fn roundtrip_all_variants() {
    for variant in [
        Ecosystem::Web,
        Ecosystem::Game,
        Ecosystem::Ai,
        Ecosystem::Cloud,
        Ecosystem::Cicd,
        Ecosystem::Iot,
        Ecosystem::App,
        Ecosystem::Lib,
        Ecosystem::Hardware,
    ] {
        let s = variant.as_str();
        assert_eq!(Ecosystem::from_str(s), Some(variant));
    }
}

#[test]
fn debug_format() {
    let s = format!("{:?}", Ecosystem::Web);
    assert_eq!(s, "Web");
}

#[test]
fn copy_works() {
    let a = Ecosystem::Ai;
    let b = a;
    assert_eq!(a, b);
}

// ── H-1 identity invariants ─────────────────────────────────────────────
// `as_str` is the canonical CLI identity and must equal `CoreIdent::core_id()`
// for all nine cores. The adapter-side half of this assertion lives in
// `mgc-types/tests/capabilities.rs` (`ecosystem_as_str_matches_core_id`) so
// the contract is pinned from both sides and a third compensating layer is
// never needed again.
// (`as_str` là identity CLI chuẩn và phải bằng `core_id()` cho cả chín core.)

#[test]
fn canonical_names_are_the_cli_core_ids() {
    // Written out literally on purpose: this list IS the CLI surface
    // (`mgc create-clo`, `ALL_CORES` in the capabilities command).
    // (Viết tay có chủ đích: danh sách này chính là bề mặt CLI.)
    let cli_core_ids = [
        "web", "game", "ai", "clo", "cicd", "iot", "app", "lib", "hardware",
    ];
    let names: Vec<&str> = Ecosystem::ALL.iter().map(Ecosystem::as_str).collect();
    assert_eq!(names, cli_core_ids);
}

#[test]
fn all_covers_every_variant_once() {
    assert_eq!(Ecosystem::ALL.len(), 9);
    let mut seen = std::collections::HashSet::new();
    for ecosystem in Ecosystem::ALL {
        assert!(seen.insert(ecosystem.as_str()), "duplicate in ALL");
    }
}

#[test]
fn persisted_values_read_back_as_canonical() {
    // Legacy `ecosystem = "cloud"` (written by an older build) must read as
    // the same ecosystem, but callers always receive the canonical name.
    for ecosystem in Ecosystem::ALL {
        let persisted = Ecosystem::from_persisted_str(ecosystem.as_str()).unwrap();
        assert_eq!(persisted, ecosystem);
        assert_eq!(persisted.as_str(), ecosystem.as_str());
    }
    let legacy = Ecosystem::from_persisted_str("cloud").unwrap();
    assert_eq!(legacy, Ecosystem::Cloud);
    assert_eq!(legacy.as_str(), "clo");
    assert_eq!(
        Ecosystem::from_persisted_str("  CLO ").unwrap().as_str(),
        "clo"
    );
    assert_eq!(Ecosystem::from_persisted_str("nope"), None);
}

#[test]
fn canonical_core_name_is_idempotent_and_single_tabled() {
    for ecosystem in Ecosystem::ALL {
        let canonical = Ecosystem::canonical_core_name(ecosystem.as_str());
        assert_eq!(canonical, ecosystem.as_str());
        assert_eq!(Ecosystem::canonical_core_name(&canonical), canonical);
    }
    assert_eq!(Ecosystem::canonical_core_name("cloud"), "clo");
    // Unknown names pass through untouched so callers can report them.
    // (Tên lạ giữ nguyên để caller báo lỗi.)
    assert_eq!(Ecosystem::canonical_core_name("nope"), "nope");
}

#[test]
fn passthrough_namespace_only_differs_where_a_host_tag_exists() {
    for ecosystem in Ecosystem::ALL {
        for role in [
            NamespaceRole::AdapterDirectory,
            NamespaceRole::TemplateLayer,
            NamespaceRole::RegistryPackage,
        ] {
            let namespace = ecosystem.passthrough_namespace(role);
            if ecosystem == Ecosystem::Cloud {
                assert_eq!(namespace, "cloud", "cloud keeps its host tag");
            } else {
                assert_eq!(namespace, ecosystem.as_str(), "identical otherwise");
            }
        }
    }
}
