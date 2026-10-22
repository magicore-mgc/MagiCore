# Build runtime detection / Nhận diện runtime build

Projects the existing manifest/runtime detector into optimizer runtime choices for Lib and App. Detection itself is infallible and returns `Unknown` when no supported manifest is found.

Chiếu manifest/runtime detector hiện có thành optimizer runtime choice cho Lib và App. Nhận diện không phát sinh lỗi và trả `Unknown` khi không có manifest được hỗ trợ.

No local error type is fabricated for this pure projection. Coverage stays with the CLI build test suite during the source pass.

Không tạo error type giả cho phép chiếu thuần này. Coverage ở bộ test build CLI trong giai đoạn source.
