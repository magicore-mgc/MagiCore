# Web registry configuration tests

These nearest-owner unit tests cover environment override selection without changing process-global environment state. The Unix and Windows cases construct a present non-UTF-8 value and ensure it fails closed instead of silently selecting the public default.

Các unit test gần owner kiểm tra cách chọn giá trị override mà không sửa environment toàn tiến trình. Ca Unix và Windows tạo giá trị đã đặt nhưng không phải UTF-8, xác nhận nó bị từ chối thay vì âm thầm chọn registry public mặc định.
