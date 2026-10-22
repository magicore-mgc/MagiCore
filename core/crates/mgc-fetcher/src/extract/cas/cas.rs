//! Stable CAS routes and extraction pipeline coordinator.
//! Các route CAS ổn định và điều phối pipeline giải nén.

use anyhow::Result;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Copy)]
pub(in crate::extract) enum ExtractionDurability {
    Durable,
    Rebuildable,
}

/// Extracts a gzip tarball into durable CAS and materializes verified copies.
/// Hashing and publication use the store's parallel batch APIs.
/// `claim` registers each imported blob against the project's generation token;
/// pass `None` when the caller has no database.
/// (Giải nén gzip tarball vào CAS durable rồi materialize bản sao đã xác minh.
/// Hash và publish dùng API batch song song của store. `claim` đăng ký mỗi blob
/// theo generation token của project; truyền `None` nếu caller không có DB.)
pub fn extract_tarball_to_cas_and_link<R: Read>(
    reader: R,
    dest: &Path,
    store: &mgc_store::ContentStore,
    claim: Option<(&mgc_store::Database, &str, i64)>,
) -> Result<()> {
    extract_inner(
        reader,
        dest,
        false,
        store,
        claim,
        ExtractionDurability::Durable,
    )
    .map(|_| ())
}

/// Extracts a lock-verified archive into a package cache that can be rebuilt
/// from that same verified archive. Callers validate the cache root before reuse.
/// (Giải nén archive đã xác minh theo lock vào cache package có thể dựng lại từ
/// archive đó. Caller phải xác minh root cache trước khi dùng lại.)
pub fn extract_tarball_to_rebuildable_cas_and_link<R: Read>(
    reader: R,
    dest: &Path,
    store: &mgc_store::ContentStore,
    claim: Option<(&mgc_store::Database, &str, i64)>,
) -> Result<()> {
    extract_inner(
        reader,
        dest,
        false,
        store,
        claim,
        ExtractionDurability::Rebuildable,
    )
    .map(|_| ())
}

/// Imports a standard npm `package/` archive into CAS and verifies an existing
/// package root without materializing a second extracted tree. Returns `false`
/// for non-standard archive roots so callers can use ordinary staging extraction.
/// (Import archive npm chuẩn `package/` vào CAS và xác minh root hiện có mà
/// không tạo cây giải nén thứ hai. Trả `false` nếu root không chuẩn để caller
/// dùng luồng staging thông thường.)
pub fn extract_tarball_to_cas_and_verify_existing_root<R: Read>(
    reader: R,
    package_root: &Path,
    store: &mgc_store::ContentStore,
    claim: Option<(&mgc_store::Database, &str, i64)>,
) -> Result<bool> {
    extract_inner(
        reader,
        package_root,
        true,
        store,
        claim,
        ExtractionDurability::Durable,
    )
}

/// Rebuildable counterpart for an existing package root whose marker and full
/// file signature the caller already validated. Missing or corrupt cache files
/// must be detected and rebuilt before reuse.
/// (Bản rebuildable cho root package đã được caller xác minh marker và chữ ký
/// file đầy đủ. File cache thiếu/hỏng phải được phát hiện và dựng lại trước khi dùng.)
pub fn extract_tarball_to_rebuildable_cas_and_verify_existing_root<R: Read>(
    reader: R,
    package_root: &Path,
    store: &mgc_store::ContentStore,
    claim: Option<(&mgc_store::Database, &str, i64)>,
) -> Result<bool> {
    extract_inner(
        reader,
        package_root,
        true,
        store,
        claim,
        ExtractionDurability::Rebuildable,
    )
}

fn extract_inner<R: Read>(
    reader: R,
    dest: &Path,
    verify_existing_package_root: bool,
    store: &mgc_store::ContentStore,
    claim: Option<(&mgc_store::Database, &str, i64)>,
    durability: ExtractionDurability,
) -> Result<bool> {
    let profiling = std::env::var_os("MAGICORE_FETCHER_PROFILE").is_some();
    let total_started = std::time::Instant::now();
    let dest_root = super::super::extract_target::ensure_extract_root(dest)?;
    let staged =
        super::super::cas_spool::spool_archive(reader, &dest_root, verify_existing_package_root)?;
    let Some(staged) = staged else {
        return Ok(false);
    };
    let file_count = staged.files.len();
    let archive_read_ms = staged.archive_read_ms;
    super::super::cas_tree::prepare_destination(&dest_root, &staged)?;
    let publish: super::super::cas_publish::PublishMetrics =
        super::super::cas_publish::publish(staged, store, claim, durability)?;
    super::super::profile::report(
        profiling,
        file_count,
        archive_read_ms,
        publish.cas_import_ms,
        publish.file_export_ms,
        total_started.elapsed().as_millis(),
    );
    Ok(true)
}
