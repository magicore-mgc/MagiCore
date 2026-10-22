//! Shared patch API wiring and exports.
//! Wiring và export API patch dùng chung.

mod lock_patch;
mod patch_spec;

pub use self::lock_patch::LockPatch;
pub use self::patch_spec::{PatchKind, PatchSpec};
