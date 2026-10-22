# App audit integration tests / Test integration audit App

This folder decomposes the Cargo auto-discovered `audit_tests` integration target by audit behavior. The root module wires focused test owners; the neighboring `tests/audit_tests.rs` remains the stable Cargo target entry.

Folder này tách integration target `audit_tests` do Cargo tự nhận diện theo từng hành vi audit. Module gốc nối các owner test; file `tests/audit_tests.rs` liền kề giữ nguyên entry của Cargo.

`test_support/` owns only temporary project directories. `flutter_audit/`, `unsupported_framework_audit/`, and `kotlin_audit/` cover the current fail-closed/unsupported reports. `manifest_aggregation/` is a status smoke check whose current assertions do not prove that every scanner ran. `swift_project_scope/` covers parent-versus-grandparent lockfile discovery. `native_lock_parsers/` separates Swift, CocoaPods and Dart lock parser fixtures.

`test_support/` chỉ sở hữu thư mục project tạm. Các owner Flutter/framework/Kotlin kiểm tra report fail-closed hoặc unsupported hiện tại. Assertion ở `manifest_aggregation/` chỉ smoke-test loại status, chưa chứng minh mỗi scanner đã chạy. `swift_project_scope/` kiểm tra lockfile ở thư mục cha và ông. `native_lock_parsers/` chia fixture parser Swift, CocoaPods và Dart.

These tests are not a claim that every App framework has a complete install/build/test lifecycle. No Cargo manifest target is added or renamed by this folder.

Các test này không khẳng định mọi framework App đã có đủ lifecycle install/build/test. Folder này không thêm hoặc đổi tên Cargo target trong manifest.
