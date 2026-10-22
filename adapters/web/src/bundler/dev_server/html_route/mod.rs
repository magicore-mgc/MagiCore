//! HTML and HMR route exports.
//! Export route HTML và HMR.
#[path = "html_route.rs"]
mod implementation;
pub(super) use self::implementation::{serve_hmr_client, serve_index};
