// Solve-local types — Kiểu dữ liệu chỉ dùng trong solve.
// Batch aliases keep resolver owner contracts explicit — Alias giúp rõ contract giữa các owner resolver.
use crate::solver::provider::ResolvedDep;
use mgc_types::{PackageId, PackageName, Version};
use std::collections::HashMap;
use std::sync::Arc;

pub(in crate::solver::resolver::solve) type SelectedPackage = (PackageName, String, Version);
pub(in crate::solver::resolver::solve) type PrefetchedVersions = HashMap<String, Arc<[Version]>>;
pub(in crate::solver::resolver::solve) type PrefetchedDependencies =
    HashMap<PackageId, Arc<[ResolvedDep]>>;
