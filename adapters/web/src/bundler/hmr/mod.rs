//! Web HMR owner wiring and compatibility surface.
//! Wiring owner HMR Web và bề mặt tương thích.

mod client_script;
mod error;
mod event;
mod manager;
mod watcher;
mod websocket;

pub use client_script::HMR_CLIENT_SCRIPT;
pub use event::HmrEvent;
pub use manager::HmrManager;
pub use websocket::hmr_ws_handler;
