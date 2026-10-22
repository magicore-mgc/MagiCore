# Lock schema owner

EN: Owns the shared lockfile write-schema decision: missing config defaults to v3, explicit `3`/`4` and `v4` retain existing handling, and unsupported values keep the current error. It does not implement a lock writer.

VI: Sở hữu quyết định schema lockfile ghi dùng chung: cấu hình thiếu mặc định v3, giá trị tường minh `3`/`4` và `v4` giữ cách xử lý hiện tại, giá trị khác giữ nguyên lỗi. Owner này không triển khai lock writer.
