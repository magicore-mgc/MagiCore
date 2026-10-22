//! Dependency Route owner wiring — nối owner dependency route.
#[path = "dependency_route.rs"]
mod implementation;
#[path = "response_payload/mod.rs"]
mod response_payload;

pub(super) use self::implementation::serve_dep;
