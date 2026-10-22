# Ecosystem detection owner

Owns project-core detection while preserving a stable public detector route. Its implementation delegates explicit identity, lock syntax validation, language-specific manifest parsing and normalized core-hint validation to named child owners. It validates malformed or linked files, rejects conflicting declarations, and never writes project configuration as a side effect. Parser/source errors retain their chains; conflict and unknown-core errors have a local typed owner with stable display text.

## Tiếng Việt

Owner nhận diện core dự án và giữ ổn định route public của detector. Implementation giao việc đọc identity tường minh, kiểm tra cú pháp lock, parse manifest theo ngôn ngữ và xác thực core hint đã chuẩn hóa cho các owner con có tên. Owner kiểm tra file lỗi hoặc symlink, từ chối khai báo xung đột và không tự ghi cấu hình dự án. Lỗi parser/source giữ nguyên chuỗi nguồn; lỗi xung đột và core không hỗ trợ có owner typed cục bộ với thông báo ổn định.
