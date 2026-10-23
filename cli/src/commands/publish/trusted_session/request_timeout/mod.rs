//! Request timeout owner.
//! Owner timeout request.

mod request_timeout;

pub(in crate::commands) use request_timeout::{
    TRUSTED_NPM_REQUEST_TIMEOUT_SECS, TRUSTED_OCI_REQUEST_MAX_TIMEOUT_SECS, docker_push_timeout,
    oci_request_timeout, trusted_npm_request_timeout,
};
