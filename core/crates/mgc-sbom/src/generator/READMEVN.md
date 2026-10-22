# Các owner generator SBOM

Facade generator giữ nguyên mgc_sbom::generator::SbomGenerator. Phép chiếu graph legacy/v4 đã xác minh, identity package, xuất JSON tiện ích và lỗi typed cục bộ nằm ở owner riêng. Bộ integration hiện tại tiếp tục là bằng chứng cho luồng public đầy đủ và che dữ liệu bí mật. Batch source-only này không đổi hành vi sản phẩm.

## Tiến độ

Xem docs/ProgressReport.md và docs/Checklist.md. Kiểm tra thực thi được hoãn tới khi đóng source pass và dung lượng máy dưới 80 GB.
