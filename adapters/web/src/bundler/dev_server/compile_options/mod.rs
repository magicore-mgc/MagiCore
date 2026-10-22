//! Compile Options owner wiring — nối owner compile options.
#[path = "compile_options.rs"]
mod implementation;
pub(super) use self::implementation::{
    COMPILER_IDENTITY, COMPILER_VERSION, canonical_compile_options,
};
