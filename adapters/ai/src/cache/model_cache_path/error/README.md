# Model path errors

This owner defines the typed validation failure returned before a model ID becomes a filesystem path. The conversion to shared `MgError` keeps the local error in the diagnostic source chain.

Owner này định nghĩa lỗi validation typed trước khi model ID trở thành filesystem path. Khi chuyển sang `MgError` dùng chung, lỗi cục bộ vẫn nằm trong source chain của diagnostic.

Nearest error-conversion case: `test/error.rs`.

Test gần logic chuyển lỗi: `test/error.rs`.
