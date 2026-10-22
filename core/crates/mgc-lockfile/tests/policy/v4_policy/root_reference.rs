// Existing v4 policy test cases: root reference.
// Test policy V4 hiện có: root reference.

#[test]
fn v4_verifier_rejects_root_pin_without_a_locked_package() {
    let mut lock = unsigned_doc();
    lock.root_dependencies_by_owner.insert(
        "lib".to_string(),
        vec!["mgc-root-v1:python:requests@2.31.0".to_string()],
    );
    refresh_digest(&mut lock);

    let error = verify_v4_math(&lock).expect_err("test operation must fail");

    assert!(error.to_string().contains("root pin"));
}

#[test]
fn v4_verifier_rejects_root_pin_ambiguous_across_sources() {
    let mut lock = unsigned_doc();
    with_python_package(&mut lock, "internal", VariantKey::default());
    with_python_package(&mut lock, "public", VariantKey::default());
    lock.root_dependencies_by_owner.insert(
        "lib".to_string(),
        vec!["mgc-root-v1:python:requests@2.31.0".to_string()],
    );
    refresh_digest(&mut lock);

    let error = verify_v4_math(&lock).expect_err("test operation must fail");

    assert!(error.to_string().contains("ambiguous"));
}

#[test]
fn v4_verifier_rejects_root_pin_ambiguous_across_variants() {
    let mut lock = unsigned_doc();
    with_python_package(&mut lock, "pypi", VariantKey::default());
    with_python_package(
        &mut lock,
        "pypi",
        VariantKey {
            peer_context: Some("peer-set-a".to_string()),
            ..VariantKey::default()
        },
    );
    lock.root_dependencies_by_owner.insert(
        "lib".to_string(),
        vec!["mgc-root-v1:python:requests@2.31.0".to_string()],
    );
    refresh_digest(&mut lock);

    let error = verify_v4_math(&lock).expect_err("test operation must fail");

    assert!(error.to_string().contains("ambiguous"));
}

#[test]
fn v4_verifier_accepts_one_unambiguous_root_pin_match() {
    let mut lock = unsigned_doc();
    with_python_package(&mut lock, "pypi", VariantKey::default());
    lock.root_dependencies_by_owner.insert(
        "lib".to_string(),
        vec!["mgc-root-v1:python:requests@2.31.0".to_string()],
    );
    refresh_digest(&mut lock);

    let report = verify_v4_math(&lock).expect("test operation must succeed");

    assert!(report.digest_ok);
}

#[test]
fn v4_verifier_resolves_legacy_npm_protocol_prefix() {
    let mut lock = unsigned_doc();
    lock.packages.push(PackageV4 {
        key: PackageKey {
            ecosystem: EcosystemTag::Web,
            name: "lodash".to_string(),
            version: "4.17.20".to_string(),
            source_id: "npm".to_string(),
            variant: VariantKey::default(),
        },
        edges: Vec::new(),
        markers: None,
        artifact: None,
        provenance: None,
        toolchain: None,
        scripts_policy: None,
        store_ref: None,
    });
    lock.root_dependencies
        .push("npm:lodash@4.17.20".to_string());
    refresh_digest(&mut lock);

    assert!(verify_v4_math(&lock).expect("test operation must succeed").digest_ok);
}

#[test]
fn v4_verifier_rejects_malformed_qualified_root_pin() {
    let mut lock = unsigned_doc();
    lock.root_dependencies_by_owner.insert(
        "lib".to_string(),
        vec!["mgc-root-v1:unknown:requests@2.31.0".to_string()],
    );
    refresh_digest(&mut lock);

    assert!(verify_v4_math(&lock).is_err());
}
