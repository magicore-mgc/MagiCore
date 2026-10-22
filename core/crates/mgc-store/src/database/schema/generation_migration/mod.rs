//! Legacy CAS generation migration wiring and phase exports.
//! Wiring migration CAS generation cũ và export các phase.
mod generation_migration;
pub(super) use generation_migration::migrate_legacy_token_schema;
#[path = "create_v2_token_tables/mod.rs"]
mod create_v2_token_tables;
#[path = "generation_token_migration_needed/mod.rs"]
mod generation_token_migration_needed;
#[path = "legacy_generation_column_present/mod.rs"]
mod legacy_generation_column_present;
#[path = "migrate_v0_token_rows/mod.rs"]
mod migrate_v0_token_rows;
#[path = "migrate_v1_token_rows/mod.rs"]
mod migrate_v1_token_rows;
