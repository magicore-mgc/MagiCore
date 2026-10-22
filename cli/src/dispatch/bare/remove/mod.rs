// Bare remove exports — expose the coordinator-facing mapper and local error.
// Export remove bare — cung cấp mapper cho coordinator và lỗi cục bộ.
mod error;
mod remove;

pub(in crate::dispatch) use remove::map;
