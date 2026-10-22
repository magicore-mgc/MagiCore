# CAS claim token validation

Owns the read gate that verifies a generation belongs to the project and is still staging before claim writes begin.

## Trách nhiệm

Quản lý cổng đọc để xác minh generation thuộc đúng project và còn staging trước khi bắt đầu ghi claim.

The caller holds the per-database writer gate and immediate transaction. Preserve error variants, SQL, and error context.

Caller giữ writer gate theo database và transaction immediate. Giữ nguyên variant lỗi, SQL và ngữ cảnh lỗi.
