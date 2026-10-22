# Shared Cargo manifest owner

This module parses and updates only the Cargo dependency tables MagiCore can represent. Its public API stays at `mgc_adapter_base::cargo_manifest`; parser, writer, table mutation and typed local errors have separate owners. Unsupported Cargo semantics continue to fail closed. Cargo is not invoked as a dependency manager; package operations stay on the MagiCore path.

Module này chỉ parse và cập nhật các bảng dependency Cargo mà MagiCore biểu diễn được. API public vẫn ở `mgc_adapter_base::cargo_manifest`; parser, writer, cập nhật table và lỗi typed có owner riêng. Semantics Cargo chưa hỗ trợ tiếp tục fail-closed. MagiCore không gọi Cargo như package manager; các thao tác package tiếp tục đi qua MagiCore.

Unsupported diagnostics derive the core label from the caller's `Ecosystem` for both parsing and mutation; lib, game and IoT therefore retain their own core identity while sharing one implementation.

Diagnostic unsupported lấy nhãn core từ `Ecosystem` của caller ở cả parse và cập nhật; lib, game và IoT giữ đúng danh tính core khi dùng chung implementation.

Nearest tests live under each owner `test/`; the external contract remains `core/crates/mgc-adapter-base/tests/cargo_manifest_safety.rs`. Runtime checks are NOT_RUN until the repository source pass is complete and machine use is below 80 GB.

Test gần nhất nằm trong `test/` của từng owner; contract bên ngoài vẫn ở `core/crates/mgc-adapter-base/tests/cargo_manifest_safety.rs`. Runtime checks NOT_RUN cho tới khi source pass hoàn tất và dung lượng máy dùng dưới 80 GB.
