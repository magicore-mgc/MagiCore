# Preflight tests

These tests pin path-traversal rejection, the required compat audit destination, redacted dry-run auditing, and the live-execution handoff. They use temporary data only and never start a child process. Authored tests remain NOT_RUN until the structural pass is complete and the machine Data volume is under the user limit.

Các test khóa việc chặn path traversal, bắt buộc audit destination cho compat, audit dry-run đã redact và bàn giao sang luồng chạy thật. Test chỉ dùng dữ liệu tạm, không khởi chạy process con. Test đã viết vẫn NOT_RUN tới khi xong refactor cấu trúc và dung lượng Data đạt giới hạn user đặt ra.
