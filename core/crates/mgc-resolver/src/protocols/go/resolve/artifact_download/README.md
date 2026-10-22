# Go artifact download

Fetches bytes only from the exact `ResolvedEntry.artifact_url` using the configured MGC HTTP client. Status and transport error conversion remain owned by the shared Go HTTP boundary; this owner must not resolve a different URL or bypass that client.

## Tiếng Việt

Chỉ tải bytes từ đúng `ResolvedEntry.artifact_url` bằng MGC HTTP client đã cấu hình. Chuyển đổi HTTP status và lỗi truyền tải thuộc boundary HTTP Go dùng chung; owner không tự resolve URL khác hoặc bypass client này.
