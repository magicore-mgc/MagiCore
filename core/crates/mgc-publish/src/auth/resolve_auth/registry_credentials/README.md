# Configured registry credentials

EN: Applies registry auth_type and selects configured token or Basic credentials.
VI: Áp dụng auth_type của registry và chọn token hoặc Basic credentials trong config.

Boundary: Existing anyhow diagnostic text and token/basic forcing behavior are preserved.
Tests: test/registry_credentials.rs covers forced Basic and missing forced-token errors.
