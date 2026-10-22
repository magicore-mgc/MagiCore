# OIDC discovery and JWKS fetch

English: Owns discovery document validation, same-origin JWKS URL checks and bounded JSON key retrieval. It delegates HTTPS, timeout, response-size and redirect policy to `mgc-oidc::fetch`.

Tiếng Việt: Sở hữu kiểm tra discovery document, URL JWKS cùng origin và tải JSON khóa có giới hạn. Chính sách HTTPS, timeout, kích thước response và redirect do `mgc-oidc::fetch` quản lý.

Public function paths remain `mgc_oidc::validate::{discover_jwks_url, fetch_jwks}`.
