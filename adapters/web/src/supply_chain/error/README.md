# Supply-chain errors

## Responsibility / Trách nhiệm
Classifies project/store read failures, age range and metadata refusals, direct-root identity and lockfile failures, and downgrade refusals. The coordinator wraps the typed local error in `MgError::Anyhow`, preserving its source chain while `Display` keeps the established user-facing diagnostic text.

Phân loại lỗi đọc project/store, ngưỡng tuổi và metadata, định danh root trực tiếp, lỗi lockfile và downgrade. Coordinator bọc lỗi typed cục bộ bằng `MgError::Anyhow` để giữ source chain; `Display` vẫn giữ nội dung diagnostic hiển thị hiện có.

## Tests / Kiểm thử
Nearest error tests preserve the existing English messages. NOT_RUN.
