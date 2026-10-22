//! Operation projection tests — test projection operation.
//! Checks serialized ownership against the existing gate for every verb.
//! Đối chiếu ownership serialize với gate hiện có cho mọi verb.

use super::super::operations_for;
use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, owner_for};

#[test]
fn each_operation_row_matches_the_dependency_gate() {
    let core = "game";
    let ecosystem = Some("bevy");
    let manifest = Some("cargo-toml");
    let rows = operations_for(core, ecosystem, manifest);

    for operation in DepOp::ALL {
        let context =
            DepContext::new(core, ecosystem, None, None, *operation).with_manifest_format(manifest);
        let expected_owner = match owner_for(&context) {
            DepOwner::Native => "mgc-native",
            DepOwner::ScaffoldOnly => "scaffold-only",
            DepOwner::Unsupported => "unsupported",
        };
        let row = &rows[operation.as_str()];
        assert_eq!(row["owner"], expected_owner);
        assert!(row["tools"].is_array());
        if expected_owner == "mgc-native" {
            assert_eq!(row["requires"], "Cargo.toml");
        }
    }
}
