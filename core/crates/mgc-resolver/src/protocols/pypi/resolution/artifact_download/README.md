# artifact_download / artifact_download

Fetches resolved artifact bytes through the shared mgc-http client and preserves the existing GET, body-read and HTTP-status error behavior.

Tải byte artifact đã resolve qua mgc-http dùng chung, giữ nguyên GET, đọc body và xử lý lỗi HTTP status.

This owner does not select a registry version or write cache files. The RegistryProtocol method remains the stable public dispatch boundary.

Owner này không chọn version registry và không ghi cache. Method RegistryProtocol vẫn là ranh giới dispatch công khai ổn định.
