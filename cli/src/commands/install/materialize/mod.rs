//! Lock replay, resolution, package materialization, summary and hooks.
//! Phát lại lock, resolve, vật chất hóa package, tổng kết và hooks.

mod materialize;

pub(super) use materialize::install_resolve_and_materialize;
