// Bare update exports — expose the coordinator-facing mapper and local error.
// Export update bare — cung cấp mapper cho coordinator và lỗi cục bộ.
mod error;
mod update;

pub(in crate::dispatch) use update::map;
