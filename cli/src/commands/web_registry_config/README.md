# Web registry configuration

Owns npm-compatible Web registry configuration and endpoint construction. It reads the override without lossy fallback, rejects a present non-UTF-8 value, validates the configured URL through the shared registry URL policy before returning it to any caller, preserves path prefixes, and keeps search/advisory endpoint behavior behind the public facade.

Quản lý cấu hình registry Web tương thích npm và tạo endpoint. Owner đọc override mà không fallback âm thầm; giá trị đã đặt nhưng không phải UTF-8 bị từ chối. URL được xác thực bằng policy registry dùng chung trước khi trả cho caller, giữ path prefix và hành vi endpoint search/advisory sau facade public.
