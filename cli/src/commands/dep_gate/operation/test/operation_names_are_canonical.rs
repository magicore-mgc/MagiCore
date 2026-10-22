//! Canonical operation names — tên operation chuẩn.
//! Keeps the ordered capability vocabulary aligned with CLI verbs.
//! Giữ vocabulary capability có thứ tự khớp verb CLI.

use crate::commands::dep_gate::DepOp;

#[test]
fn all_dependency_operations_keep_their_canonical_names_and_order() {
    let expected = [
        (DepOp::Install, "install"),
        (DepOp::Add, "add"),
        (DepOp::Remove, "remove"),
        (DepOp::Update, "update"),
        (DepOp::List, "list"),
        (DepOp::Resolve, "resolve"),
        (DepOp::Lock, "lock"),
        (DepOp::Fetch, "fetch"),
        (DepOp::Verify, "verify"),
        (DepOp::Store, "store"),
        (DepOp::Materialize, "materialize"),
        (DepOp::FrozenInstall, "frozen-install"),
        (DepOp::OfflineReinstall, "offline-reinstall"),
        (DepOp::Gc, "gc"),
    ];

    let expected_operations = expected
        .iter()
        .map(|(operation, _)| *operation)
        .collect::<Vec<_>>();
    assert_eq!(DepOp::ALL, expected_operations.as_slice());
    for (operation, expected_name) in expected {
        assert_eq!(operation.as_str(), expected_name);
    }
}
