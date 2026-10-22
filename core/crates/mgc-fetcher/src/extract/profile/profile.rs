//! Preserve the opt-in CAS extraction profile output.
//! Giữ output profile CAS extraction có thể bật/tắt.

pub(in crate::extract) fn report(
    enabled: bool,
    file_count: usize,
    archive_read_ms: u64,
    cas_import_ms: u128,
    file_export_ms: u128,
    total_ms: u128,
) {
    if enabled {
        eprintln!(
            "{}",
            format_profile_line(
                file_count,
                archive_read_ms,
                cas_import_ms,
                file_export_ms,
                total_ms,
            )
        );
    }
}

pub(in crate::extract) fn format_profile_line(
    file_count: usize,
    archive_read_ms: u64,
    cas_import_ms: u128,
    file_export_ms: u128,
    total_ms: u128,
) -> String {
    format!(
        "[magicore:fetcher-profile] files={} archive_read_ms={} cas_import_batch_ms={} file_export_batch_ms={} total_ms={}",
        file_count, archive_read_ms, cas_import_ms, file_export_ms, total_ms
    )
}
