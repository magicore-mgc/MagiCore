# Project trust settings owner

This owner stores the project-level allowlist of signing key IDs accepted by required lock signature policy. Local machine keyrings are not represented here. `TrustConfig` keeps its established public path through the parent re-export.

The empty key list remains the serde default. This DTO does not perform signature checks, so verification errors stay with the lock verifier and no local error type is invented.

# Owner cấu hình trust của project

Owner này lưu allowlist key ID chữ ký được chấp nhận ở cấp project khi policy yêu cầu chữ ký lock. Keyring cục bộ của máy không nằm ở đây. `TrustConfig` giữ public path hiện tại qua re-export của parent.

Danh sách key rỗng vẫn là serde default. DTO này không verify chữ ký; lỗi verify thuộc lock verifier và không tạo error type giả.
