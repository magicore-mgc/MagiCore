# Flutter package metadata

## Ownership

This folder owns MGC-authored `package_config.json` and `package_graph.json` generation for the native Flutter install lane. It does not resolve or fetch packages and does not own Swift installation. The parent install module keeps the historical helper names as a compatibility boundary.

## Extension and safety

Keep Dart serialization fields and graph invariants explicit. Registry archives and Flutter SDK packages have separate identity and filesystem checks. Path URI encoding handles relative project roots and the Windows short-path case. Changes to generated metadata need fixture coverage and an actual Dart/Flutter consumer check in the later verification phase.

The package-config, package-graph, registry-entry, SDK-entry, and project-URI owners each define a local error enum and convert at their `mod.rs` facade to the existing `MgError` contract. Existing behavior tests live under the corresponding `test/` folders; graph output is covered through the package-config owner tests. Runtime tests are pending the repository-wide verification phase.

## Ownership / Chủ sở hữu

Thư mục này sở hữu việc tạo `package_config.json` và `package_graph.json` do MGC quản lý cho lane Flutter native. Thư mục không resolve/fetch package và không sở hữu install Swift. Module install cha giữ tên helper cũ làm ranh giới tương thích.

## Mở rộng và an toàn

Giữ rõ field serialization Dart và invariant của graph. Archive registry và package Flutter SDK có kiểm tra danh tính/filesystem riêng. URI path xử lý project root tương đối và trường hợp Windows short-path. Thay đổi metadata sinh ra cần fixture và kiểm tra consumer Dart/Flutter thật ở phase verify sau.

Các owner package-config, package-graph, registry-entry, SDK-entry và project-URI có enum lỗi cục bộ riêng; `mod.rs` ánh xạ tại facade về hợp đồng `MgError` hiện tại. Test hành vi nằm trong `test/` tương ứng; test output graph đi qua owner package-config. Test runtime được để lại cho phase verify toàn repository.
