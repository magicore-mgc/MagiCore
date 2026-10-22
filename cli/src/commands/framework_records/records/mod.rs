//! Per-core record module wiring — nối module record từng core.
#[path = "ai/mod.rs"]
mod ai;
#[path = "app/mod.rs"]
mod app;
#[path = "cicd/mod.rs"]
mod cicd;
#[path = "cloud/mod.rs"]
mod cloud;
#[path = "game/mod.rs"]
mod game;
#[path = "hardware/mod.rs"]
mod hardware;
#[path = "records.rs"]
mod implementation;
#[path = "iot/mod.rs"]
mod iot;
#[path = "lib/mod.rs"]
mod lib;
#[path = "web/mod.rs"]
mod web;

pub use self::implementation::RECORDS;
