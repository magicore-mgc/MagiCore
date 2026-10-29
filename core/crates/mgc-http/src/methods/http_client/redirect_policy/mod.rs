//! Wires the redirect policy and its nearest tests.
//! Nối policy redirect và test gần nhất.

mod redirect_policy;
pub(crate) use redirect_policy::no_redirect_policy;
pub use redirect_policy::{
    ALLOW_CROSS_ORIGIN_REDIRECTS_ENV, DEFAULT_MAX_REDIRECTS, same_origin_policy,
};
#[cfg(test)]
mod test;
