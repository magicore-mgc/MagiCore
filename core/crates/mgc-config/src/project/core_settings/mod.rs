//! Per-core project configuration DTOs.
//! Các DTO cấu hình project thuộc riêng từng core.

#[path = "ai.rs"]
mod ai;
#[path = "app.rs"]
mod app;
#[path = "cicd.rs"]
mod cicd;
#[path = "cloud.rs"]
mod cloud;
#[path = "game.rs"]
mod game;
#[path = "iot.rs"]
mod iot;
#[path = "lib.rs"]
mod lib;

pub use ai::AiConfig;
pub use app::AppConfig;
mod ci;

pub use ci::CiConfig;
pub use cicd::CicdConfig;
pub use cloud::CloudConfig;
pub use game::GameConfig;
pub use iot::IotConfig;
pub use lib::LibConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
