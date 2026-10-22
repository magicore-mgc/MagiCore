# CocoaPods client error tests / Test lỗi client CocoaPods

Tests assert the existing shared error category and exact English message for checksum, request/status/body, and malformed JSON failures. App-level successful fetch and checksum mismatch fixtures are in `adapters/app/tests/native_rn_tests.rs`; this batch did not run them.

Test xác nhận nhóm lỗi dùng chung và thông báo tiếng Anh chính xác cho checksum, request/status/body và JSON lỗi. Fixture fetch thành công và checksum lệch ở tầng app nằm trong `adapters/app/tests/native_rn_tests.rs`; batch này chưa chạy chúng.
