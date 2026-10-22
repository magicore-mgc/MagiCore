//! Ephemeral trusted identity token.
// Token tạm gắn package, repository identity, generation và expiry.

use crate::auth::User;
use mgc_oidc::OidcClaims;
use std::time::Instant;

#[derive(Clone)]
pub(in crate::auth) struct TrustedToken {
    pub(in crate::auth) user: User,
    pub(in crate::auth) package: String,
    pub(in crate::auth) identity: OidcClaims,
    pub(in crate::auth) sigstore_oidc_token: String,
    pub(in crate::auth) binding_generation: u64,
    pub(in crate::auth) expires_at: Instant,
}
