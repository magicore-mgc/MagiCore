//! HttpClient behavior owners and exports.
//! Các owner hành vi và export của HttpClient.

mod bounded_response;
mod error;
mod http_client;
pub mod redirect_policy;
mod request_configuration;
mod request_execution;
mod request_methods;

pub use bounded_response::BoundedResponseError;
pub use http_client::HttpClient;
pub use redirect_policy::{
    ALLOW_CROSS_ORIGIN_REDIRECTS_ENV, DEFAULT_MAX_REDIRECTS, same_origin_policy,
};
pub(crate) use redirect_policy::no_redirect_policy;
