# Dart package configuration

Owns validation and atomic publication of `.dart_tool/package_config.json` and `.dart_tool/package_graph.json`. Registry package rows and Flutter SDK package rows have separate owners and share a single duplicate-name set so identity conflicts fail closed.

`error.rs` classifies unsafe output paths, missing roots, filesystem and serialization failures, while child-owner errors retain their original shared category. The public helper remains `MgResult<()>` at this facade; coordinator tests are in `test/`.

## Chủ sở hữu

Sở hữu kiểm tra và công bố atomic hai file `.dart_tool/package_config.json` và `.dart_tool/package_graph.json`. Row package registry và Flutter SDK có owner riêng, cùng dùng một tập chống trùng tên để xung đột danh tính bị chặn.

`error.rs` phân loại đường dẫn output không an toàn, thiếu root, lỗi filesystem và serialization; lỗi từ owner con giữ nguyên category chung. Helper tại facade vẫn trả `MgResult<()>`; test điều phối nằm trong `test/`.
