# Web Go backend build / Build backend Go của Web

## Responsibility / Trách nhiệm

Owns the Web Go backend build operation: freshness checks, MGC global module/build cache selection, safe output selection, constrained Go environment and allowlisted compiler invocation. The generic Web framework engine remains in `../../web_engine/`.

Sở hữu thao tác build backend Go của Web: freshness check, chọn module/build cache toàn cục của MGC, chọn output an toàn, giới hạn environment Go và gọi compiler qua allowlist. Engine framework Web tổng quát vẫn ở `../../web_engine/`.

## Errors and tests / Lỗi và test

The owner-local `error.rs` defines the Go toolchain-missing and Go compiler-failure diagnostics while preserving executor errors in `source()`. Freshness and safe output-path failures stay in their dedicated owners. Existing focused tests remain unchanged and are deferred until the repository source pass closes.

`error.rs` cục bộ định nghĩa diagnostic thiếu Go toolchain và lỗi compiler Go, giữ lỗi executor trong `source()`. Freshness và lỗi output path an toàn vẫn ở owner chuyên trách. Test hiện tại giữ nguyên, chờ source pass đóng.