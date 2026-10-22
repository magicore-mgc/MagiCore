//! Wire registration request, endpoint and token response owners.
//! Nối owner request đăng ký, endpoint và token response.

mod constants;
mod error;
mod registry_user_creation;
mod registry_user_url;
mod token_response;

pub(super) use error::{RegistryUserCreationError, into_diagnostic as registration_diagnostic};
pub(crate) use registry_user_creation::create_registry_user;
pub(super) use registry_user_creation::create_registry_user_inner;
