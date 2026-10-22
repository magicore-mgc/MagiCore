// adapter_factory owner wiring — nối owner adapter_factory.
// The implementation stays in its same-name module file.
use super::*;
#[path = "adapter_factory.rs"]
mod implementation;
#[cfg(any(feature = "game", feature = "clo"))]
pub(crate) use implementation::core_adapter;
