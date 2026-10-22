# Import module

Parses supported legacy package-manager lockfiles as data and exposes the stable migration API. It does not invoke or wrap another package manager.

Module nhập lockfile cũ theo dữ liệu, cung cấp API migration ổn định và không chạy hay bao gói package manager khác.

The coordinator owns file selection, dispatch, normalization, source-lockfile provenance and `ImportReport`; `detection/` owns filename discovery and trust-downgrade checks; `package_reference/` owns scoped package/version parsing; each format folder owns only its parser.

Coordinator quản lý chọn file, dispatch, chuẩn hóa và báo cáo; `detection/` quản lý nhận diện và cảnh báo trust-downgrade; `package_reference/` xử lý tên/version scoped; mỗi folder format chỉ sở hữu parser của format đó.

Behavior tests remain registered in `../../tests/import.rs` during the production-source pass. They cover format parsing, unsupported records, deterministic ordering and detection. The `import/` owner also checks that the actual source lockfile name is recorded and accepted by the v3 completeness gate.

Test hành vi hiện vẫn được đăng ký ở `../../tests/import.rs` trong lượt tách source; bao phủ parser, record không hỗ trợ, thứ tự xác định và nhận diện.

Each format parser extracts the package facts actually present in its input. The import coordinator then records the source lockfile basename as provenance; it does not invent missing registry URLs or reinterpret absent package data. Format-version numbers only drive warnings where the parser already supports shape-first input.

Parser từng format chỉ lấy dữ kiện package có thật trong input. Coordinator import sau đó ghi basename lockfile nguồn làm provenance; không tự bịa registry URL bị thiếu hay diễn giải lại dữ liệu không có. Version format chỉ tạo cảnh báo ở các parser vốn chấp nhận theo cấu trúc.
