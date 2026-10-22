//! Wire the token response decoder.
//! Nối bộ giải mã token response.

#[path = "token_response.rs"]
mod token_response;

pub(in crate::commands::login) use token_response::parse_token_response;
