//! Cargo target facade; included fragments stay at the original crate root.
//! Facade Cargo; fragment include giữ tại crate root cũ.

include!("policy/v4_policy/support.rs");
include!("policy/v4_policy/verification.rs");
include!("policy/v4_policy/root_reference.rs");
include!("policy/v4_policy/mode_resolution.rs");
include!("policy/v4_policy/enforcement.rs");
include!("policy/v4_policy/signing.rs");
include!("policy/v4_policy/file_verification.rs");
