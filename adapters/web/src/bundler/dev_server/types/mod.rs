//! Types owner wiring — nối owner types.
#[path = "types.rs"]
mod implementation;
pub use self::implementation::DevServerConfig;
pub(super) use self::implementation::ServerState;
