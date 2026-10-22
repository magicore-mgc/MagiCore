//! Registry authentication tests grouped by policy and transport behavior.
//! Test Registry auth được chia theo policy và transport.

use super::*;

#[path = "scoped_account_policy.rs"]
mod scoped_account_policy;
#[path = "scoped_account_routes.rs"]
mod scoped_account_routes;
#[path = "trusted_token_authorization.rs"]
mod trusted_token_authorization;
#[path = "trusted_token_lifecycle.rs"]
mod trusted_token_lifecycle;
#[path = "trusted_transport.rs"]
mod trusted_transport;
