# Bun lock parser errors

English: Owns typed malformed-JSON and malformed-entry failures. Converting to `MgError` retains the exact legacy `Other(String)` messages; serde errors remain available as a source before conversion.

Tiếng Việt: Sở hữu lỗi có kiểu cho JSON và entry sai cấu trúc. Khi chuyển sang `MgError`, thông điệp `Other(String)` cũ được giữ nguyên; lỗi serde vẫn là source trước boundary chuyển đổi.
