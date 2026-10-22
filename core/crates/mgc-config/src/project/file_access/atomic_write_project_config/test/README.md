# Atomic config writer tests / Test bộ ghi config atomic

These tests protect creation, replacement, permission preservation and refusal to write through a symlink. Cleanup-on-failure ordering is checked from source; no deterministic mid-write failure hook is provided.

Các test này bảo vệ tạo mới, thay thế, giữ permission và từ chối ghi qua symlink. Thứ tự dọn staging khi lỗi được rà từ source; không có hook ổn định để ép lỗi giữa lúc ghi.
