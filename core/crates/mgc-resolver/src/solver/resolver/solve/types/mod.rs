// Solve-local type wiring — Kết nối module chứa kiểu riêng của solve.
mod queued_dependency;
mod types;
pub(super) use queued_dependency::QueuedDependency;
pub(super) use types::{PrefetchedDependencies, PrefetchedVersions, SelectedPackage};
