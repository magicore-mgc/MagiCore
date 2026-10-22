//! Hardware scaffold ownership — ownership scaffold Hardware.
//! Only the implemented read-only inventory is represented as scaffold-only.
//! Chỉ inventory read-only đã triển khai mang owner scaffold-only.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, owner_for};

#[test]
fn hardware_list_is_scaffold_only_and_mutations_are_unsupported() {
    let list = DepContext::new("hardware", None, None, None, DepOp::List);
    assert!(matches!(owner_for(&list), DepOwner::ScaffoldOnly));

    for operation in [DepOp::Install, DepOp::Add, DepOp::Remove, DepOp::Update] {
        let context = DepContext::new("hardware", None, None, None, operation);
        assert!(matches!(owner_for(&context), DepOwner::Unsupported));
    }
}
