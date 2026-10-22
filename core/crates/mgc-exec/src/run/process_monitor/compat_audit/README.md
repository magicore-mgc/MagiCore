# Compatibility audit

Owns deduplicated audit recording for package-manager processes observed inside a user-declared compatibility scope. The writer checks the configured manager, redacts argv before persistence, records each process/command pair once, and surfaces a typed poison/write failure while retaining the write source. `error.rs`, `test/` and `docs/` are local to this owner.

# Audit compatibility

Sở hữu việc ghi audit chống trùng cho package manager được quan sát trong phạm vi compatibility do user khai báo. Writer đối chiếu manager, redact argv trước khi lưu, chỉ ghi một lần cho mỗi cặp process/command và trả lỗi poisoned/write có kiểu, giữ source của lỗi ghi. `error.rs`, `test/` và `docs/` thuộc riêng owner này.
