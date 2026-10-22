// Wires the types owner; behavior lives in types.rs.
// Nối owner types; xử lý nằm trong types.rs.
mod error;
mod types;
pub(super) use self::error::ModelOptionsError;
pub use self::types::{ModelArgs, ModelCmd};
