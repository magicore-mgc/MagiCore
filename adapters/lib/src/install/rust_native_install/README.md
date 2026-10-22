# Native Rust install

Owns online/offline routing for Cargo-native install and keeps one local error domain. `artifact_cache/` reads no-follow regular files and verifies registry SHA-256; `offline_install/` performs read-only full-graph preflight and verified replay; `online_install/` downloads, verifies, imports to CAS and materializes source.

## Chủ sở hữu

Sở hữu điều phối install Cargo-native online/offline và giữ một error domain cục bộ. `artifact_cache/` đọc file thường không theo link và xác minh SHA-256 registry; `offline_install/` preflight toàn graph chỉ đọc rồi replay có xác minh; `online_install/` tải, xác minh, import CAS và materialize source.

Cache and offline behavior tests live inside their respective owners. They reuse Rust archive fixtures from `install/test/support.rs`; current function names, error mapping, order and caller routes remain stable.

Test cache và offline nằm cạnh owner tương ứng. Test dùng fixture archive Rust ở `install/test/support.rs`; tên hàm, ánh xạ lỗi, thứ tự chạy và đường caller hiện tại được giữ nguyên.

`error.rs` owns the local cache/filesystem taxonomy and maps to the prior shared `MgError` categories.

`error.rs` sở hữu taxonomy cache/filesystem cục bộ và ánh xạ về các loại `MgError` cũ.

Owner-level progress and acceptance evidence are kept under `docs/` while this refactor is active.

Tiến độ và tiêu chí nghiệm thu riêng của owner được ghi trong `docs/` khi refactor còn hoạt động.
