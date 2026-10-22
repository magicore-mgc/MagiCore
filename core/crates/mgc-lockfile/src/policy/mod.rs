//! Lock v4 trust policy + verification (design §3).
//! Chính sách tin cậy + xác minh lock v4.
//!
//! Principle (stated plainly so it cannot be overclaimed): a lone hash
//! is NOT anti-tamper — anyone who can rewrite the lock can recompute
//! it. Authenticity needs a signature verified against a trusted key.
//! READMEs, CLIs and docs must never call an unsigned lock
//! "zero-trust" or "tamper-proof".
//! (Nguyên tắc: hash đơn lẻ KHÔNG chống tamper. Chỉ chữ ký verify được
//! với key tin cậy mới cho authenticity.)

mod digest;
mod enforcement;
mod file_verification;
mod install_integrity;
mod mode;
mod policy;
mod report;
mod signing;
mod verification;

pub use enforcement::enforce_policy;
pub use file_verification::verify_v4_file;
pub use install_integrity::{INSTALL_SRI_ALGORITHMS, check_install_sri, native_sha256_hex};
pub use mode::LockPolicyMode;
pub use policy::resolve_policy;
pub use report::V4VerifyReport;
pub use signing::sign_payload_digest;
pub use verification::verify_v4_math;
