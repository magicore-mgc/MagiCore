# App Swift manifest owners

Owns static `Package.swift` dependency parsing, `Package.resolved` pin projection and the explicit unsupported writer. Public paths remain `manifest::swift::{parse_package_swift,write_package_swift,scan_package_swift_text}`. The parser never executes SwiftPM; computed source shapes fail closed.

Sở hữu parse dependency tĩnh `Package.swift`, projection pin `Package.resolved` và writer báo không hỗ trợ. Giữ các đường dẫn public nêu trên. Parser không chạy SwiftPM; shape source tính toán bị từ chối fail-closed.

`package_source/` owns regular-file/static-shape handling; `resolved_pins/` owns pin reading and registry/Git matching. Existing `MgError` contracts remain; no duplicate local error type is introduced. Nearest-owner tests live under each module's `test/`; public lifecycle integration tests remain under `adapters/app/tests/`.

`package_source/` xử lý file thường/shape tĩnh; `resolved_pins/` sở hữu đọc pin và khớp registry/Git. Giữ contract `MgError` cũ, không tạo error type trùng. Test gần owner ở `test/`; integration lifecycle public giữ tại `adapters/app/tests/`.
