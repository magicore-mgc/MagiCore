# Finalize upload / Hoàn tất upload

Owns the final PUT request, digest query construction, status response and Location extraction.

Quản lý request PUT cuối, dựng query digest, status response và đọc Location.

Digest and path pieces are currently interpolated into the URL string. Correct encoding/validation is an open behavior/security task.

Digest và các phần path hiện được nội suy vào chuỗi URL. Encode/validate đúng là đầu việc behavior/security còn mở.

