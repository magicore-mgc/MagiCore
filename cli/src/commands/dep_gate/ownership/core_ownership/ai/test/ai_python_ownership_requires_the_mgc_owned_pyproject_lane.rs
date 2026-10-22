//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn ai_python_ownership_requires_the_mgc_owned_pyproject_lane() {
    for op in DepOp::ALL {
        assert!(
            matches!(
                owner_for(&ctx("ai", Some(eco::PYTHON), *op)),
                DepOwner::Unsupported
            ),
            "ai/python/{:?} must not be advertised without an owned manifest",
            op
        );
        let owned = DepContext::new("ai", Some(eco::PYTHON), None, None, *op)
            .with_manifest_format(Some("mgc-pyproject"));
        let expected_native = !matches!(op, DepOp::Gc);
        assert_eq!(
            matches!(owner_for(&owned), DepOwner::Native),
            expected_native,
            "ai/python/{op:?} ownership must match implemented operations"
        );
    }
}
