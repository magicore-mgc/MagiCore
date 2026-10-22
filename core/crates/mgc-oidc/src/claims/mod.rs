//! OIDC claim type facade — facade kiểu claim OIDC.
//! Public `claims::{Audience, OidcClaims}` paths stay stable.

mod audience;
mod oidc_claims;

pub use audience::Audience;
pub use oidc_claims::OidcClaims;
