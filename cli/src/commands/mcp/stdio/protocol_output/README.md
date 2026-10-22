# MCP protocol output handle

Owns the platform-specific duplication, redirection and restoration of protocol stdout. The saved handle remains RAII-owned and the unsupported-platform failure stays fail-closed.

## Phạm vi

Folder này sở hữu việc nhân đôi, chuyển hướng và khôi phục stdout giao thức theo hệ điều hành. Handle đã lưu vẫn do RAII quản lý và nhánh platform chưa hỗ trợ vẫn fail-closed.
