# Cache errors / Lỗi cache

Defines package-cache-specific error categories and keeps the established English diagnostics for lock poison, missing entries and integrity mismatch.
Định nghĩa nhóm lỗi riêng của cache package và giữ nguyên diagnostic tiếng Anh hiện có cho lock poison, thiếu entry và integrity mismatch.

Filesystem and platform causes continue through `anyhow` so their source chain is retained.
Lỗi filesystem/platform tiếp tục đi qua `anyhow` để giữ chuỗi nguyên nhân.
