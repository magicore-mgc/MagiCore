//! Server owner wiring — nối owner server.
#[path = "server.rs"]
mod implementation;
pub use self::implementation::MgDevServer;
