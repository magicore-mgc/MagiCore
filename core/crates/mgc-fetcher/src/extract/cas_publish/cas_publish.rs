//! Import staged blobs, publish copies and fail closed on claim errors.
//! Import blob staging, publish bản sao và fail closed khi claim lỗi.

use super::super::cas::ExtractionDurability;
use super::super::cas_spool::StagedArchive;
use super::super::{ExtractionError, ExtractionResult};
use std::time::Instant;

pub(in crate::extract) struct PublishMetrics {
    pub(in crate::extract) cas_import_ms: u128,
    pub(in crate::extract) file_export_ms: u128,
}

pub(in crate::extract) fn publish(
    staged: StagedArchive,
    store: &mgc_store::ContentStore,
    claim: Option<(&mgc_store::Database, &str, i64)>,
    durability: ExtractionDurability,
) -> ExtractionResult<PublishMetrics> {
    let has_claim = claim.is_some();
    let cas_import_started = Instant::now();
    let import_result = match durability {
        ExtractionDurability::Durable => store.import_files_with_exec_batch(
            staged
                .files
                .iter()
                .map(|file| (file.source_path.as_path(), file.executable)),
        ),
        ExtractionDurability::Rebuildable => store.import_rebuildable_files_with_exec_batch(
            staged
                .files
                .iter()
                .map(|file| (file.source_path.as_path(), file.executable)),
        ),
    };
    let hashes =
        import_result.map_err(|error| ExtractionError::CasImport(anyhow::Error::new(error)))?;
    let cas_import_ms = cas_import_started.elapsed().as_millis();

    let mut imported = std::collections::HashSet::new();
    if has_claim {
        // Refcount keys use bare BLAKE3 hex; executable state stays in CAS address.
        // Khóa refcount dùng BLAKE3 hex thuần; trạng thái executable nằm trong địa chỉ CAS.
        imported.extend(hashes.iter().map(|hash| hash.as_hex().to_string()));
    }

    let file_export_started = Instant::now();
    let destinations = staged
        .files
        .into_iter()
        .zip(hashes.iter().cloned())
        .map(|(file, hash)| (hash, file.path));
    let export_result = match durability {
        ExtractionDurability::Durable => store.export_batch_to(destinations),
        ExtractionDurability::Rebuildable => store.export_batch_to_rebuildable_root(destinations),
    };
    export_result.map_err(|error| ExtractionError::CasExport(anyhow::Error::new(error)))?;
    let file_export_ms = file_export_started.elapsed().as_millis();

    // Gate 11-B.2: a kill here leaves published blobs without this generation's claim.
    // Gate 11-B.2: kill tại đây để blob đã publish nhưng chưa có claim generation này.
    mgc_store::failpoint::hit("after-cas-publish");
    if let Some((db, project_root, generation)) = claim {
        let mut hashes: Vec<String> = imported.into_iter().collect();
        hashes.sort_unstable();
        let hash_refs: Vec<&str> = hashes.iter().map(String::as_str).collect();
        if !hashes.is_empty() {
            // Fail closed: promotion retires claims already present; it cannot rebuild
            // a missing graph-to-blob claim. Missing claims can make prune delete live data.
            // Fail closed: promote chỉ nghỉ hưu claim sẵn có, không dựng lại mapping
            // graph-to-blob. Claim thiếu có thể khiến prune xóa dữ liệu đang dùng.
            db.cas_claim_batch(project_root, generation, &hash_refs)
                .map_err(|error| ExtractionError::Claim(anyhow::Error::new(error)))?;
            // Gate 11-B.2: a kill here leaves a claim-bearing generation; its
            // classifier decides STALE versus RETAINED from the promoted refset.
            // Gate 11-B.2: kill tại đây để generation có claim; classifier quyết
            // định STALE hay RETAINED dựa trên refset đã promote.
            mgc_store::failpoint::hit("after-first-claim");
        }
    }
    Ok(PublishMetrics {
        cas_import_ms,
        file_export_ms,
    })
}
