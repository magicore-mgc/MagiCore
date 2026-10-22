//! Build verification owner wiring — guards the existing MGC build route.
//! Wiring owner xác minh build — bảo vệ route build MGC hiện có.
mod build_verification;
mod error;
pub(super) use build_verification::verify_with_build;
