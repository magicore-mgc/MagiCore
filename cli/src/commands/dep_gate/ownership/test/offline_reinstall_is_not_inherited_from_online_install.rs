//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn offline_reinstall_is_not_inherited_from_online_install() {
    for (core, ecosystem) in [
        ("app", Some(eco::SWIFT)),
        ("lib", Some(eco::GO)),
        ("lib", Some(eco::JAVA)),
        ("lib", Some(eco::DOTNET)),
        ("game", Some(eco::BEVY)),
        ("iot", Some("esp32-rust")),
    ] {
        assert!(
            matches!(
                owner_for(&ctx(core, ecosystem, DepOp::OfflineReinstall)),
                DepOwner::Unsupported
            ),
            "{core}/{ecosystem:?} must not inherit online install as offline support"
        );
        assert!(
            gate(
                &ctx(core, ecosystem, DepOp::OfflineReinstall),
                None,
                &native(),
                None
            )
            .is_err()
        );
    }

    let lib_python_native = DepContext::new(
        "lib",
        Some(eco::PYTHON),
        None,
        None,
        DepOp::OfflineReinstall,
    )
    .with_manifest_format(Some("pep621-native"));
    assert!(matches!(owner_for(&lib_python_native), DepOwner::Native));
    assert!(gate(&lib_python_native, None, &native(), None).is_ok());

    let lib_rust_native =
        DepContext::new("lib", Some(eco::RUST), None, None, DepOp::OfflineReinstall)
            .with_manifest_format(Some("cargo-toml"));
    assert!(matches!(owner_for(&lib_rust_native), DepOwner::Native));
    assert!(gate(&lib_rust_native, None, &native(), None).is_ok());
    assert!(matches!(
        owner_for(&lib_ctx(
            eco::RUST,
            Some("maven-pom"),
            DepOp::OfflineReinstall
        )),
        DepOwner::Unsupported
    ));

    let lib_python_foreign = DepContext::new(
        "lib",
        Some(eco::PYTHON),
        None,
        None,
        DepOp::OfflineReinstall,
    )
    .with_manifest_format(Some("requirements-txt"));
    assert!(matches!(
        owner_for(&lib_python_foreign),
        DepOwner::Unsupported
    ));

    let ai_python_native =
        DepContext::new("ai", Some(eco::PYTHON), None, None, DepOp::OfflineReinstall)
            .with_manifest_format(Some("mgc-pyproject"));
    assert!(matches!(owner_for(&ai_python_native), DepOwner::Native));
    assert!(gate(&ai_python_native, None, &native(), None).is_ok());

    for (core, ecosystem) in [
        ("app", Some(eco::FLUTTER)),
        ("web", Some(eco::JS)),
        ("web", Some(eco::TS)),
        ("lib", Some(eco::TS)),
    ] {
        assert!(matches!(
            owner_for(&ctx(core, ecosystem, DepOp::OfflineReinstall)),
            DepOwner::Native
        ));
    }
    let cloud_cdk = DepContext::new(
        "clo",
        Some(eco::JS),
        Some("cdk"),
        None,
        DepOp::OfflineReinstall,
    );
    assert!(matches!(owner_for(&cloud_cdk), DepOwner::Unsupported));
}
