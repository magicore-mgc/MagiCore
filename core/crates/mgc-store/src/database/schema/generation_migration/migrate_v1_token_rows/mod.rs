//! migrate v1 token rows module wiring.
//! Wiring module migrate v1 token rows.
#[path = "migrate_v1_token_rows.rs"]
mod migrate_v1_token_rows;
pub(super) use migrate_v1_token_rows::migrate_v1_token_rows;
