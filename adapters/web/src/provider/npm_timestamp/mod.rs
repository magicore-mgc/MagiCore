//! npm_timestamp owner wiring.
//! Wiring owner parse timestamp npm.
mod npm_timestamp;
pub(crate) use npm_timestamp::parse_npm_time;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
