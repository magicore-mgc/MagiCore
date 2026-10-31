// Per-core engine adapter exports — expose the engine command mapper.
// Export của adapter engine theo core — cung cấp mapper lệnh engine.
mod engine_folder;

pub(in crate::dispatch) use engine_folder::engine_to_common;
