//! CAS promotion implementation module wiring.
//! Wiring các module triển khai promote CAS.
#[path = "flip_generation_state/mod.rs"]
mod flip_generation_state;
#[path = "promote_impl.rs"]
mod promote_impl;
#[path = "retire_older_generations/mod.rs"]
mod retire_older_generations;
#[path = "verify_staging_token/mod.rs"]
mod verify_staging_token;
