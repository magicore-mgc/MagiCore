# Hardware build owner / Owner build Hardware

## Responsibility / Trách nhiệm

Rejects PlatformIO projects until MagiCore has a native, dependency-owned build backend. It must not invoke `pio run` or silently route to another core.

Từ chối project PlatformIO tới khi MagiCore có backend build native sở hữu dependency. Không chạy `pio run` hoặc âm thầm định tuyến sang core khác.

## Errors and tests / Lỗi và test

The owner-local `error.rs` defines PlatformIO build refusal and missing-Hardware-project errors. It keeps the current CLI text and never invokes `pio run`. Tests remain in the CLI test owner until the source pass closes.

`error.rs` cục bộ định nghĩa lỗi từ chối build PlatformIO và không tìm thấy project Hardware. Diagnostic CLI giữ nguyên và không gọi `pio run`. Test giữ ở CLI test owner tới khi đóng source pass.