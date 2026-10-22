//! Cargo target facade; included fragments stay at the original crate root.
//! Facade Cargo; fragment include giữ tại crate root cũ.

include!("policy/v4_sri_digest_shape/support.rs");
include!("policy/v4_sri_digest_shape/sri_shape.rs");
include!("policy/v4_sri_digest_shape/migration_integrity.rs");
include!("policy/v4_sri_digest_shape/native_sha256.rs");
