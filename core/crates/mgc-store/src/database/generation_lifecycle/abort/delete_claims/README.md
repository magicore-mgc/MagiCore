# Abort claim deletion

Owns deletion of the claim rows belonging to one verified staging generation.

## Trách nhiệm

Quản lý việc xóa các row claim thuộc một staging generation đã xác minh.

The caller verifies state and transaction lifetime. Preserve project/generation scoping, SQL, and error mapping.

Caller xác minh state và vòng đời transaction. Giữ nguyên phạm vi project/generation, SQL và ánh xạ lỗi.
