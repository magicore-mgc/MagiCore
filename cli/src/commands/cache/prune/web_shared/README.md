# Web shared cache pruning

## Responsibility

Coordinates package-root enumeration, live reference reads, size accounting and unpinned-root deletion. It preserves the existing root marker, schema-version, missing-project cleanup and empty-directory cleanup behavior.

## Trách nhiệm

Điều phối liệt kê package root, đọc reference còn sống, tính dung lượng và xóa root không được ghim. Giữ nguyên hành vi marker, schema version, dọn reference có project đã mất và dọn thư mục rỗng.
