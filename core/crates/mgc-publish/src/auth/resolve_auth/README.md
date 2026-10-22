# Credential resolution

EN: Coordinates the existing source precedence and exposes the stable resolve_auth entry point.
VI: Điều phối thứ tự ưu tiên nguồn hiện có và cung cấp entry point resolve_auth ổn định.

Boundary: CLI token, environment, npmrc, and configured registry credentials keep the same order and diagnostics. The module does not persist or log credentials.
Tests: The owner tests exercise the public resolution entry point; source-specific rules are tested by their child owners.
