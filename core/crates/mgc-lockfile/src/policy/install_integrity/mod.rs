//! Install-time SRI compatibility exports.
//! Export kiểm tra SRI cho install.

mod install_integrity;

pub use install_integrity::{INSTALL_SRI_ALGORITHMS, check_install_sri, native_sha256_hex};
