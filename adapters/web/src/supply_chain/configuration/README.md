# Supply-chain configuration

## Purpose / Chức năng
Reads the Web release-age override from project configuration with the existing database fallback, and parses existing boolean environment switches.

Đọc tuổi phát hành Web từ cấu hình dự án với fallback database hiện có, đồng thời parse các cờ boolean môi trường.

## Boundaries / Ranh giới
This owner preserves current configuration precedence and error handling. The project value is currently passed through as if it were seconds even though SecurityConfig documents hours; that unit mismatch is an explicit follow-up, not hidden by this extraction.

Owner giữ nguyên precedence và cách bắt lỗi hiện tại. Giá trị project hiện được dùng như giây trong khi SecurityConfig ghi đơn vị giờ; sai lệch đơn vị này được ghi thành đầu việc tiếp theo, không bị che bởi lần tách này.

## Tests / Kiểm thử
The configured-age reader and environment truth table each have a nearest test owner. Their tests are authored but NOT_RUN.
