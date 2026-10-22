//! Environment token source owner / owner nguồn token từ environment.

mod environment_token;
#[cfg(test)]
mod test;

pub(in crate::auth::resolve_auth) use environment_token::resolve_environment_token;
