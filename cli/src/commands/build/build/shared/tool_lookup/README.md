# Build tool lookup / Dò tool build

## Responsibility / Trách nhiệm

Own PATH lookup with Windows PATHEXT handling and the platform-specific Python launcher name. The helper only detects availability; it never downloads or installs a tool.

Sở hữu dò PATH có xử lý PATHEXT trên Windows và tên Python launcher theo nền tảng. Helper chỉ nhận diện tool có sẵn, không tải hay cài tool.

## Errors and tests / Lỗi và test

`tool_unavailable` returns a boolean and intentionally has no error type. Existing lookup tests remain in `cli/src/test/build_test.rs`; the test phase will place them with this owner after production-source closure.

`tool_unavailable` trả boolean nên không có error type. Test dò tool còn ở `cli/src/test/build_test.rs`; chuyển test về owner sau khi đóng source production.
