# Start upload / Bắt đầu upload

Builds the session-initiation URL, sends the initial POST, and extracts the upload identifier from Location. It owns start-specific status/header errors.

Dựng URL tạo session, gửi POST đầu tiên và đọc mã upload từ Location. Owner này quản lý lỗi status/header riêng của bước khởi tạo.

The POST can currently be retried by HttpClient. Duplicate session creation remains an open behavior risk.

POST hiện có thể bị HttpClient retry. Nguy cơ tạo session trùng vẫn là gap behavior cần xử lý.

