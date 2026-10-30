# source_readiness

EN: Completes a materialized Cargo registry source so BOTH readers accept it: the verified `.cargo-checksum.json` record (MagiCore build gate and later installs) and Cargo's `.cargo-ok` readiness marker. `require_cargo_source_marker` reports a directory Cargo would silently re-extract, and `source_root` is the single place that builds `registry/src/{name}-{version}`.

VI: Hoàn tất source registry Cargo đã materialize để CẢ HAI bên đọc chấp nhận: record `.cargo-checksum.json` đã xác minh (cổng build MagiCore và các lần install sau) và marker `.cargo-ok` của Cargo. `require_cargo_source_marker` báo thư mục mà Cargo sẽ âm thầm giải nén lại, và `source_root` là nơi duy nhất dựng `registry/src/{name}-{version}`.
