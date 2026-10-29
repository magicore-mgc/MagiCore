//! Shared HTTP client public facade.
//! Facade công khai cho HTTP client dùng chung.

mod http_client;

pub use http_client::{
    ALLOW_CROSS_ORIGIN_REDIRECTS_ENV, BoundedResponseError, DEFAULT_MAX_REDIRECTS, HttpClient,
    same_origin_policy,
};
