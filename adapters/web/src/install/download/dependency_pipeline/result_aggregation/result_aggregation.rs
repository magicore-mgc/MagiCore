//! Aggregate finished package pipeline tasks.
//! Gom kết quả task package đã hoàn tất.

use std::collections::HashMap;
use std::path::PathBuf;

use mgc_types::{MgResult, PackageId};
use tokio::task::JoinHandle;

pub(super) type PipelineTaskOutput = (u64, PackageId, PathBuf, Option<JoinHandle<()>>);
pub(in crate::install::download::dependency_pipeline) type AggregatedPipelineResults =
    (u64, HashMap<PackageId, PathBuf>, Vec<JoinHandle<()>>);

pub(in crate::install::download::dependency_pipeline) fn aggregate_pipeline_results(
    finished: Vec<MgResult<PipelineTaskOutput>>,
) -> MgResult<AggregatedPipelineResults> {
    let mut total_bytes = 0u64;
    let mut results = HashMap::new();
    let mut persist_handles = Vec::new();
    let mut pipeline_errors = Vec::new();
    for joined in finished {
        match joined {
            Ok((bytes, id, root, persist)) => {
                total_bytes += bytes;
                results.insert(id, root);
                if let Some(persist) = persist {
                    persist_handles.push(persist);
                }
            }
            Err(error) => pipeline_errors.push(error),
        }
    }
    if let Some(error) = pipeline_errors.into_iter().next() {
        return Err(error);
    }
    Ok((total_bytes, results, persist_handles))
}

#[cfg(test)]
#[path = "test/result_aggregation.rs"]
mod tests;
