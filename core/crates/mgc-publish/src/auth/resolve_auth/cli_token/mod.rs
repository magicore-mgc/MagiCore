//! Explicit CLI token owner / owner token CLI tường minh.

mod cli_token;
#[cfg(test)]
mod test;

pub(in crate::auth::resolve_auth) use cli_token::resolve_cli_token;
