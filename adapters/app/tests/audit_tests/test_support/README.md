# Test support / Hỗ trợ test

Owns the single `tmp(tag)` helper used to create an App audit fixture directory scoped by process and test thread, preventing tests that reuse a tag from deleting each other's fixtures during parallel execution. Its visibility stays inside the integration-test crate.

Sở hữu helper duy nhất `tmp(tag)` để tạo thư mục fixture audit App theo process và test thread, tránh test trùng tag xóa fixture của nhau khi chạy song song. Visibility chỉ trong integration-test crate.
