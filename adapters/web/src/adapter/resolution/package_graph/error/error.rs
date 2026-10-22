//! Typed package-graph refusals / từ chối graph package có kiểu.

/// Errors that prevent exact resolver roots from becoming graph roots.
/// Lỗi ngăn root resolver chính xác trở thành root của graph.
#[derive(Debug, thiserror::Error)]
pub(in crate::adapter::resolution::package_graph) enum PackageGraphError {
    #[error("resolver direct root '{package_id}' is absent from its resolved package set")]
    MissingResolvedRoot { package_id: String },
}
