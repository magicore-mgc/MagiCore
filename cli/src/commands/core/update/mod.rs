//! `mgc update` — router: core detect → file con (v5: LỆNH = folder, CORE = file).

#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "cicd")]
pub mod cicd;
#[cfg(feature = "clo")]
pub mod clo;
#[cfg(feature = "game")]
pub mod game;
#[cfg(feature = "hardware")]
pub mod hardware;
#[cfg(feature = "iot")]
pub mod iot;
#[cfg(feature = "lib")]
pub mod library;
pub mod web;

// Router `run(core, ...)` cũ đã bị loại; package dispatch hiện nằm tại `dispatch::core::package_ops`.
// Legacy `run(core, ...)` router is retired; package dispatch now lives in `dispatch::core::package_ops`.
