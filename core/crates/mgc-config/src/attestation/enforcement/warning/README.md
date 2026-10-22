# Attestation warning gate / Cổng cảnh báo attestation

Owns the process-wide once-only warning gate shared by live-marker and missing-marker enforcement.

Quản lý cổng cảnh báo một lần mỗi process được dùng chung bởi enforcement marker hiện hữu và marker bị thiếu.

The small claim operation is isolated so its single-use behavior can be tested without mutating process environment or emitting terminal output.

Thao tác claim nhỏ được tách riêng để kiểm tra tính dùng một lần mà không sửa environment process hoặc ghi output terminal.
