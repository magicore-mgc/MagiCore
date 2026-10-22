// Per-core engine adapter exports — expose the engine command mapper.
// Export của adapter engine theo core — cung cấp mapper lệnh engine.
mod engine;

pub(in crate::dispatch) use engine::engine_to_common;
