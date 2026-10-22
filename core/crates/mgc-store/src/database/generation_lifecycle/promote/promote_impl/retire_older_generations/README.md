# Older generation retirement

Owns the two ordered SQL cleanup phases for claims belonging to older CAS generations.

## Trách nhiệm

Quản lý hai bước dọn SQL theo đúng thứ tự cho claim thuộc các CAS generation cũ.

The caller holds the immediate transaction and has verified the token. Preserve SQL text, order, staging protection, and typed error mapping.

Caller giữ transaction immediate và đã xác minh token. Giữ nguyên SQL, thứ tự, bảo vệ staging và ánh xạ lỗi có type.
