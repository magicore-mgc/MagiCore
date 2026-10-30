# materialize

EN: Cargo cache/source directory materialization. `MaterializeError` distinguishes filesystem IO from archive extraction errors and restores the original shared error category at the facade.

VI: Materialize thư mục cache/source Cargo. `MaterializeError` phân biệt IO filesystem với lỗi giải nén archive rồi giữ nguyên nhóm lỗi chung tại facade.

EN: Extraction replaces a stale source tree and always finishes with Cargo's `source_readiness` marker (`.cargo-ok`), so a materialized directory is complete for Cargo, not only checksum-recorded for MagiCore.

VI: Giải nén thay thế cây source cũ và luôn kết thúc bằng marker `source_readiness` của Cargo (`.cargo-ok`), nên thư mục đã materialize là hoàn tất với Cargo, không chỉ có checksum record cho MagiCore.
