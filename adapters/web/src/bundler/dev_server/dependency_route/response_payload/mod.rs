//! Dependency response-payload wiring.
//! Wiring payload response dependency.

#[path = "response_payload.rs"]
mod implementation;

pub(super) use self::implementation::{css_injector, missing_dependency_diagnostic};
