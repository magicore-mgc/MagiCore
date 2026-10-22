//! Source Route owner wiring — nối owner source route.
#[path = "source_route.rs"]
mod implementation;
pub(super) use self::implementation::serve_source_or_static;
