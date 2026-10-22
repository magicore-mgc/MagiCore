//! `mgc dev` — router: core detect → file con (v5: LỆNH = folder, CORE = file).
//! Port cố định per core từ bảng tập trung `dev_port.rs` (RULE §13 — hoán vị 4·3·1·5).

#[cfg(feature = "ai")]
#[path = "ai/mod.rs"]
pub mod ai;
#[cfg(feature = "app")]
#[path = "app/mod.rs"]
pub mod app;
#[cfg(feature = "cicd")]
#[path = "cicd/mod.rs"]
pub mod cicd;
#[cfg(feature = "clo")]
#[path = "clo/mod.rs"]
pub mod clo;
#[cfg(feature = "game")]
#[path = "game/mod.rs"]
pub(crate) mod game;
#[cfg(feature = "iot")]
#[path = "iot/mod.rs"]
pub mod iot;
#[path = "web/mod.rs"]
pub mod web;

// Router chung nằm tại commands::dev; các module tại đây sở hữu lifecycle theo core.
// Shared routing lives in commands::dev; this tree owns per-core lifecycles.
