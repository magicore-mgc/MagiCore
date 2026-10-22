//! Typed path and deletion failures for Web store pruning.
//! Lỗi path và xóa có type cho prune Web store.

#[derive(Debug, thiserror::Error)]
pub(super) enum PruneError {
    #[error(transparent)]
    RootSafety(#[from] super::root_safety::RootSafetyError),
    #[error(transparent)]
    CandidatePlan(#[from] super::candidate_plan::CandidatePlanError),
    #[error("remove package directory {path}: {source}")]
    RemovePackageDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("logical payload report total overflow after pruning package {package}")]
    CompletedPayloadOverflow { package: String },
    #[error("Web store root changed while acquiring the project lock: {path}")]
    StoreRootChanged { path: String },
}
