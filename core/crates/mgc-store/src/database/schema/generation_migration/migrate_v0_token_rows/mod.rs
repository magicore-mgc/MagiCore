//! migrate v0 token rows module wiring.
//! Wiring module migrate v0 token rows.
#[path = "migrate_v0_token_rows.rs"]
mod migrate_v0_token_rows;
pub(super) use migrate_v0_token_rows::migrate_v0_token_rows;
