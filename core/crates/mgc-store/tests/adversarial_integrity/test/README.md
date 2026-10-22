# Behavior tests / Test theo hành vi

Each module owns adversarial assertions for one area: import validation, quarantine, export, memo verification, concurrency, compiled cache or filesystem boundaries. The root target retains all original `#[test]` and platform `#[cfg]` registrations.

Mỗi module sở hữu assertion đối kháng cho một nhóm: validate import, quarantine, export, memo xác minh, concurrency, compiled cache hoặc ranh giới filesystem. Target gốc giữ nguyên toàn bộ đăng ký `#[test]` và `#[cfg]` theo nền tảng.
