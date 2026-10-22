//! npmrc credentials owner / owner credentials từ npmrc.

mod npmrc_credentials;
#[cfg(test)]
mod test;

pub(in crate::auth::resolve_auth) use npmrc_credentials::resolve_npmrc_credentials;
