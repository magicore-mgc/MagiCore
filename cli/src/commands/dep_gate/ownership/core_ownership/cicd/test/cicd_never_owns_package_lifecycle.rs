//! CI/CD dependency policy — policy dependency CI/CD.
//! Pipeline scaffolding does not imply package lifecycle ownership.
//! Tạo pipeline không đồng nghĩa sở hữu package lifecycle.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, owner_for};

#[test]
fn every_package_operation_remains_unsupported_for_cicd() {
    for operation in DepOp::ALL {
        let context = DepContext::new("cicd", None, None, None, *operation);
        assert!(matches!(owner_for(&context), DepOwner::Unsupported));
    }
}
