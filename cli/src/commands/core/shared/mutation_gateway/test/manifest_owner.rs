//! Native manifest and update ownership policy contract.
//! Contract policy ownership manifest và update native.

use super::super::{ensure_native_manifest_owner, ensure_native_update_owner};

#[test]
fn shared_dependency_mutations_refuse_toolchain_owned_paths() {
    assert!(ensure_native_manifest_owner(true, "add").is_ok());

    let add_error = ensure_native_manifest_owner(false, "add").unwrap_err();
    assert!(
        add_error
            .to_string()
            .contains("native MagiCore manifest owner")
    );

    let remove_error = ensure_native_manifest_owner(false, "remove").unwrap_err();
    assert!(
        remove_error
            .to_string()
            .contains("native MagiCore manifest owner")
    );

    assert!(ensure_native_update_owner(true).is_ok());
    let update_error = ensure_native_update_owner(false).unwrap_err();
    assert!(
        update_error
            .to_string()
            .contains("native MagiCore update engine")
    );
}
