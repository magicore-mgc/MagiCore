//! Destination safety wiring.
//! Wiring an toàn destination.

mod real_directory;
mod safe_path;
pub(super) use real_directory::ensure_real_directory;
pub(super) use safe_path::ensure_safe_model_destination;
