//! Transpile owner wiring — nối owner transpile.
#[path = "transpile.rs"]
mod implementation;
pub(super) use self::implementation::serve_transpiled;
