//! Core catalog behavior tests — test hành vi catalog core.
//! Locks canonical order and the unknown-core diagnostic.
//! Khóa thứ tự chuẩn và thông báo core không biết.

use super::super::{ALL_CORES, capabilities_for_core};

#[test]
fn canonical_core_order_is_stable() {
    assert_eq!(
        ALL_CORES,
        [
            "web", "lib", "ai", "app", "game", "iot", "clo", "cicd", "hardware"
        ]
    );
}

#[test]
fn unknown_core_keeps_the_legacy_diagnostic() {
    let error = capabilities_for_core("not-a-core").err().unwrap();
    assert_eq!(error.to_string(), "Unknown core 'not-a-core'");
}

// ── Advertised matrix vs per-project reality ────────────────────────────
// `mgc capabilities` describes a CORE, while an adapter narrows its claims per
// language/project shape. Printing only the base profile under-reported what a
// real project performs, so these cases pin the union property.
// (Matrix mô tả CORE trong khi adapter thu hẹp theo ngôn ngữ/project. In mỗi bản
// gốc báo thấp hơn thực tế, nên các case này ghim thuộc tính hợp.)

use mgc_types::capabilities::Capability;

/// Every capability a narrower profile can grant must appear in the union.
///
/// A language profile that grants something the matrix omits turns
/// `mgc capabilities` into a lie in the *safe-looking* direction: a caller
/// decides `lib` cannot resolve, and writes its own resolver.
/// (Mọi capability mà profile hẹp có thể cấp phải có trong hợp. Thiếu nó biến
/// `mgc capabilities` thành lời nói sai theo hướng "trông an toàn": caller tưởng
/// `lib` không resolve được và tự viết resolver.)
#[test]
fn the_lib_union_covers_every_language_profile() {
    let union = mgc_lib_adapter::LibAdapter::CAPABILITIES_ALL;
    for profile in [
        mgc_lib_adapter::LibAdapter::CAPABILITIES,
        mgc_lib_adapter::LibAdapter::CAPABILITIES_TS,
        mgc_lib_adapter::LibAdapter::CAPABILITIES_JAVA_DELEGATED,
        mgc_lib_adapter::LibAdapter::CAPABILITIES_PYTHON_UNOWNED,
    ] {
        for capability in profile {
            assert!(
                union.contains(capability),
                "{capability:?} is granted by a lib language profile but missing from \
                 CAPABILITIES_ALL, so `mgc capabilities` under-reports it"
            );
        }
    }
}

/// The AI union covers both the base set and the PYPI lane.
/// (Hợp AI phủ cả bản gốc lẫn lane PYPI.)
#[test]
fn the_ai_union_covers_the_pypi_lane() {
    let union = mgc_ai_adapter::AiAdapter::CAPABILITIES_ALL;
    for profile in [
        mgc_ai_adapter::AiAdapter::CAPABILITIES,
        mgc_ai_adapter::AiAdapter::CAPABILITIES_PYPI,
    ] {
        for capability in profile {
            assert!(
                union.contains(capability),
                "{capability:?} is granted by an ai profile but missing from CAPABILITIES_ALL"
            );
        }
    }
}

/// The union carries no duplicate entries, so a printed list reads once each.
/// (Hợp không có mục trùng để danh sách in ra đọc một lần mỗi mục.)
#[test]
fn the_unions_have_no_duplicates() {
    for union in [
        mgc_lib_adapter::LibAdapter::CAPABILITIES_ALL,
        mgc_ai_adapter::AiAdapter::CAPABILITIES_ALL,
    ] {
        let mut seen = std::collections::BTreeSet::new();
        for capability in union {
            assert!(
                seen.insert(format!("{capability:?}")),
                "{capability:?} appears twice"
            );
        }
    }
}

/// The CLI must report the union, not the base profile.
/// (CLI phải báo hợp, không phải bản gốc.)
#[test]
fn the_cli_reports_the_union_for_lib_and_ai() {
    for (core, expected) in [
        ("lib", Capability::DependencyResolver),
        ("ai", Capability::DependencyResolver),
        ("lib", Capability::ArtifactFetcher),
        ("ai", Capability::ArtifactFetcher),
    ] {
        let caps = super::super::capabilities_for_core(core).expect("known core");
        assert!(
            caps.contains(&expected),
            "`mgc capabilities --core {core}` must report {expected:?}, got {caps:?}"
        );
    }
}
