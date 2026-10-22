# Secure identity endpoint transport

English: Owns HTTPS URL validation and bounded JSON transport. Redirects remain disabled; connect/request/deadline timeout and response cap remain centralized in `limits`.

Tiếng Việt: Sở hữu kiểm tra URL HTTPS và transport JSON có giới hạn. Redirect tiếp tục bị tắt; timeout kết nối/request/deadline và trần response nằm ở `limits`.

`secure_url` and `fetch_json` retain crate-internal `mgc_oidc::fetch::*` routes for OIDC validation. Tests do not make network requests.
