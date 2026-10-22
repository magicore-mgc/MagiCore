//! Verify the stable public root and module error paths.
//! Kiểm tra public path root và module ổn định cho API lỗi.

use mgc_types::error::{MgError as ModuleMgError, MgResult as ModuleMgResult};
use mgc_types::{MgError, MgResult};

#[test]
fn root_and_module_paths_share_the_same_types() {
    let root_error: MgError = MgError::Other("shared".to_owned());
    let module_error: ModuleMgError = root_error;
    let root_result: MgResult<()> = Err(module_error);
    let module_result: ModuleMgResult<()> = Ok(());

    assert!(matches!(root_result, Err(MgError::Other(message)) if message == "shared"));
    assert!(matches!(module_result, Ok(())));
}
