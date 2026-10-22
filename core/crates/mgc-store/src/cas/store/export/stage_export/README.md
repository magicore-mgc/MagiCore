# Stage Export owner

Stage from one already-open source handle, select descriptor COW or plain-copy fallback, and report the strategy actually used. The copy fallback rewinds a cloned file handle and never reopens the source path. `StageExportError` retains the underlying I/O source locally and maps to the existing `StoreError` at the export boundary.

Stage từ một source handle đã mở, chọn COW descriptor hoặc fallback copy thường và báo chiến lược thực sự đã dùng. Nhánh copy rewind handle được clone và không mở lại source path. `StageExportError` giữ source I/O ở owner và map về `StoreError` hiện có tại ranh giới export.

Nearest source test: `test/stage_export.rs`; it exercises this owner implementation directly. Runtime status: NOT_RUN.
