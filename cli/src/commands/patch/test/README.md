# Patch command tests / Test lệnh patch

Tests are grouped by `add`, `remove`, and `verify`; `support.rs` holds the shared project fixture. Existing 10 test bodies and their `cfg(unix)`/Tokio attributes are moved unchanged.

Test được gom theo `add`, `remove`, `verify`; `support.rs` giữ fixture project dùng chung. Giữ nguyên 10 thân test cùng attribute `cfg(unix)`/Tokio. They are NOT_RUN in the source-only phase.

Các test này NOT_RUN trong phase chỉ refactor source.
