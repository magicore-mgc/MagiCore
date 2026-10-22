//! Warm install fast-path owner wiring.
//! Wiring owner fast-path install đã có sẵn.

mod fast_path;
pub(super) use fast_path::try_noop;
