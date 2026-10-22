# Resolver errors

EN: Centralizes the existing authentication-resolution diagnostic constructors without changing the public anyhow error type or message text.
VI: Tập trung các constructor chẩn đoán của resolver xác thực, giữ nguyên kiểu anyhow công khai và nội dung thông báo.

Tests: test/error.rs asserts exact rendered diagnostics for missing credentials and forced auth modes.
