# Minimum-release-age install policy

This owner decides whether a locked graph can be reused when a configured release-age gate is active. It checks the core adapter's timestamp-aware resolver capability and refuses `--frozen` reuse when the lock lacks publication timestamps. The shared `core::shared::*` facade preserves callers. `age_gate.rs` owns the decision/config lookup; `error.rs` preserves the two existing refusal diagnostics as local typed variants; `test/` contains the moved focused policy tests. Adapter-specific timestamp resolution stays in each adapter/resolver. No benchmark is applicable to this small deterministic policy check.

# Policy minimum-release-age cho install

Owner này quyết định có thể dùng lại locked graph khi release-age gate được cấu hình. Nó kiểm tra resolver theo timestamp của adapter core và từ chối reuse với `--frozen` khi lock không lưu publication timestamp. Facade `core::shared::*` giữ nguyên caller. `age_gate.rs` sở hữu quyết định/đọc cấu hình; `error.rs` giữ nguyên hai thông báo từ chối dưới dạng variant typed cục bộ; `test/` chứa test policy được chuyển về gần module. Resolve timestamp theo từng adapter vẫn thuộc adapter/resolver. Không áp dụng benchmark cho policy nhỏ và xác định này.
