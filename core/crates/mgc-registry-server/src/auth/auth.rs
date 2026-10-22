//! Stable public authentication and authorization API for the registry server.
//! API xác thực và phân quyền public ổn định của registry server.
pub use super::middleware::authentication::authentication::auth_middleware;
pub use super::middleware::middleware::unauthorized_response;
pub use super::middleware::optional::optional::optional_auth;
pub use super::middleware::trusted_transport::trusted_transport::trusted_transport_middleware;
pub use super::policy::access::access::{check_publish_access, check_scope_access};
pub use super::policy::policy::scope_matches;
pub use super::request::authentication::authentication::extract_auth;
pub use super::request::user_context::user_context::get_user;
pub use super::types::types::{AuthService, User, UserRole};
