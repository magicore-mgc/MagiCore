# OIDC fetch

English: This facade preserves `mgc_oidc::fetch::*`. The implementation is split among transport, HTTP limits, environment/file token sources, GitHub Actions requests and the public coordinator.

Tiếng Việt: Facade này giữ nguyên `mgc_oidc::fetch::*`. Implementation được chia theo transport, giới hạn HTTP, nguồn token env/file, request GitHub Actions và coordinator công khai.

The crate-wide `OidcError` remains the shared error contract. No token is returned in diagnostics. The existing CLI trusted-publish caller remains external and unchanged.
