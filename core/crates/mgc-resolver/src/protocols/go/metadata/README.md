# Go metadata owner

Parses proxy `.info` and `@latest` JSON, then checks that the reported version matches the requested or locked identity.

## Tiếng Việt

Parse JSON `.info` và `@latest` từ proxy, sau đó xác minh version trả về khớp định danh được yêu cầu hoặc ghi trong lock. Lỗi malformed/identity mismatch tiếp tục là `MgError::Integrity`. Integration cases ở resolver test suite.
