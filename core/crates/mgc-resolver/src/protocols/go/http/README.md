# Go proxy HTTP owner

Owns GOPROXY/SUMDB URL construction, status/body handling, checksum lookup and verified artifact transfer. Public `GoModProtocol::download_module` remains on the stable type.

## Tiếng Việt

Quản lý tạo URL GOPROXY/SUMDB, xử lý status/body, tra checksum và tải artifact đã ghim. API `GoModProtocol::download_module` giữ nguyên. Các kiểm thử HTTP thực tế nằm trong integration suite `core/crates/mgc-resolver/tests/protocols_go.rs`. Dùng `MgError::Network`/`Integrity` hiện có và giữ fail-closed.
