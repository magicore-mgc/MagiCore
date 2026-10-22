//! Behavior-focused test implementation modules.
//! Module implementation test được nhóm theo hành vi.

#[path = "concurrency.rs"]
pub(crate) mod concurrency;
#[path = "digest_framing.rs"]
pub(crate) mod digest_framing;
#[path = "generation_claims.rs"]
pub(crate) mod generation_claims;
#[path = "key_context.rs"]
pub(crate) mod key_context;
#[path = "record_validation.rs"]
pub(crate) mod record_validation;
#[path = "root_security.rs"]
pub(crate) mod root_security;
#[path = "threat_boundary.rs"]
pub(crate) mod threat_boundary;
#[path = "verification_memo.rs"]
pub(crate) mod verification_memo;
