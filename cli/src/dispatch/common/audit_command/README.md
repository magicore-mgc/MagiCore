# Audit command owner

Owns the split between package audit and signature audit. Signature audit rejects the existing incompatible `fix`/`format` flags before invoking the handler. It uses the shared CLI diagnostic and `anyhow::Result`; no local error type is needed.

## Tiếng Việt

Owner phân tuyến audit package và audit chữ ký. Audit chữ ký từ chối cặp flag `fix`/`format` không tương thích trước khi gọi handler. Owner dùng diagnostic CLI và `anyhow::Result` chung, không cần error type cục bộ.
