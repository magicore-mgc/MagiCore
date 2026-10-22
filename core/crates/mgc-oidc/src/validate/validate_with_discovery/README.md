# Discovery validation coordinator

English: Owns the call order between input limits, JWT header parsing, unverified issuer extraction, exact allowlist matching, JWKS cache retrieval and full token verification.

Tiếng Việt: Sở hữu thứ tự giữa giới hạn input, parse JWT header, đọc issuer chưa xác minh, so khớp allowlist chính xác, lấy JWKS cache và xác minh token đầy đủ.

The issuer must match the configured string before any network discovery. Public registry-server callers remain on the existing API.
