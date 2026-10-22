# Dart package graph

Builds a Dart Pub-compatible graph from the root `pubspec.yaml`, MGC's resolved packages and selected Flutter SDK packages. The small `package_graph.rs` coordinates the owners; `mod.rs` keeps the parent-visible builder and local error conversion.

## Chủ sở hữu

Dựng graph tương thích Dart Pub từ `pubspec.yaml` gốc, package MGC đã resolve và package Flutter SDK được chọn. `package_graph.rs` điều phối các owner; `mod.rs` giữ builder mà parent đang dùng và chuyển lỗi local.

- `root_manifest/` owns root identity/version and sorted root dependency names.
- `resolved_packages/` projects MGC packages; duplicate names now fail with the existing `DuplicatePackage` integrity error instead of overwriting an earlier version in the name-keyed map.
- `sdk_packages/` owns canonical SDK manifest lookup, identity validation and representable dependency forms.
- `edge_validation/` verifies root and package edges after all package owners complete.
- `error/` stays shared at this boundary because all behavior owners return the same `FlutterPackageGraphError` taxonomy and preserve one `MgError` conversion; local tests check integrity classification and source-chain retention.
- Each owner has its own `test/README.md` and focused source tests. The existing end-to-end package-config test remains at `package_config/test/`.

`root_manifest/` sở hữu identity/version root và tên dependency đã sắp xếp.

`resolved_packages/` projection package MGC; tên trùng bị từ chối bằng lỗi integrity `DuplicatePackage` có sẵn thay vì ghi đè version trước trong map theo name.

`sdk_packages/` sở hữu tìm manifest SDK canonical, xác thực identity và xử lý dạng dependency biểu diễn được.

`edge_validation/` xác minh edge root/package sau khi ráp xong toàn bộ owner package.

`error/` được chia sẻ tại boundary này vì các owner cùng trả taxonomy `FlutterPackageGraphError` và giữ một nơi chuyển sang `MgError`; test cục bộ bảo vệ phân loại integrity.

Mỗi owner có `test/README.md` và test source tập trung riêng. Test package-config end-to-end hiện hữu vẫn ở `package_config/test/`.

Thirteen focused source test functions are authored but NOT_RUN. The existing package-config integration test remains unchanged and has not run. Compiler, runtime, formatter and integration verification remain pending global source closure and the machine-storage gate.

Đã viết 13 test source tập trung nhưng NOT_RUN. Integration test package-config hiện hữu được giữ nguyên và chưa chạy. Compile, runtime, formatter và integration verification chờ đóng source toàn repo và đạt gate dung lượng máy.
