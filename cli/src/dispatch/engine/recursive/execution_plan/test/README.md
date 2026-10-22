# Execution-plan tests

These tests resolve workspace identities and reject a later explicit-core mismatch for the complete plan. The coordinator's source ordering calls this planner before its handler loop; these unit tests exercise planning only. They read temporary project configuration and parse CLI values.

## Tiếng Việt

Các test này resolve identity workspace và từ chối mismatch core ở workspace sau trong toàn bộ plan. Source coordinator gọi planner trước vòng handler; unit test chỉ kiểm tra bước lập plan. Test chỉ đọc cấu hình project tạm và CLI value đã parse.
