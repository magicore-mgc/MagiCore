//! Balanced outer quote parser.
//! (Parser quote ngoài cân bằng.)

mod env_quote;

pub(super) use env_quote::strip_env_quotes;

#[cfg(test)]
#[path = "test/env_quote.rs"]
mod tests;
