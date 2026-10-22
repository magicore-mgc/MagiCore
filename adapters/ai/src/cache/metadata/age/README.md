# Cache-entry age

This owner calculates whole cached/idle days and preserves the existing behavior that future timestamps produce zero elapsed days.

Owner này tính số ngày trọn kể từ lúc cache/lần truy cập và giữ hành vi cũ: timestamp tương lai cho elapsed days bằng không.

Nearest cases are in `test/age.rs`.

Test gần nhất nằm ở `test/age.rs`.
