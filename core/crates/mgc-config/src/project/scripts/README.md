# Project script policy owner

This owner defines the committed `[scripts]` DTO, the single deny-first decision function shared by package installation and `mgc trust pending`, and typed loading errors. It preserves the historical reason strings and missing-file/table behavior. Malformed policy remains fail-closed and keeps its cause in `ScriptsPolicyError`.

`project.rs` re-exports the established public names. Its `load_scripts_table` wrapper keeps the existing `String` error signature for compatibility; internal CLI and adapter paths use `load_scripts_table_typed` so the typed source chain reaches their diagnostic boundary. No caller may duplicate the decision order.

The owner catches only read/TOML/value-conversion failures it can classify. It does not turn broken policy into `None` or success. Focused tests live in `test/`; the existing crate and adapter integration tests remain in place.

# Owner policy script của project

Owner này định nghĩa DTO `[scripts]` đã commit, hàm quyết định deny-first duy nhất dùng chung cho cài package và `mgc trust pending`, cùng lỗi typed khi đọc policy. Owner giữ nguyên reason string và hành vi khi thiếu file/bảng. Policy lỗi vẫn fail-closed và giữ nguyên nguyên nhân trong `ScriptsPolicyError`.

`project.rs` re-export các public name cũ. Wrapper `load_scripts_table` giữ signature lỗi `String` để tương thích; CLI và adapter nội bộ dùng `load_scripts_table_typed` để giữ source chain đến ranh giới diagnostic. Caller không được tự nhân bản thứ tự quyết định.

Owner chỉ bắt các lỗi đọc/TOML/chuyển đổi giá trị có thể phân loại. Không biến policy lỗi thành `None` hoặc success. Test tập trung nằm trong `test/`; integration test hiện hữu của crate và adapter vẫn giữ nguyên.
