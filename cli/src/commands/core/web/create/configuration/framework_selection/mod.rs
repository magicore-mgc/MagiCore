//! Wires the framework-selection implementation and its nearest tests.
//! Nối implementation chọn framework và test gần nhất.

#[path = "framework_selection.rs"]
mod implementation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::commands::core::web::implementation) use self::implementation::{
    detect_backend_framework, enforce_framework_language_defaults, resolve_framework,
};
