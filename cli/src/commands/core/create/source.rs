//! Carry offline-source flags from the command boundary into the config.
//! Chuyển cờ nguồn offline từ biên lệnh vào config.

use crate::commands::core::shared::source_flags::SourceFlags;
use crate::wizard::engine::ScaffoldConfig;

/// Apply `--from-local` / `--no-local` to a scaffold config.
/// Áp `--from-local` / `--no-local` vào config scaffold.
pub(crate) fn apply(config: &mut ScaffoldConfig, source: SourceFlags) {
    config.from_local = source.from_local;
    config.no_local = source.no_local;
}
