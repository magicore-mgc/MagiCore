# Lib native engine integration test owners

This folder contains the behavior owners behind Cargo's auto-discovered
`native_engine_tests` integration target. The target root keeps the stable
`#[tokio::test]` registrations; each owner stores the matching test body and
protocol-specific fixtures. `support/` contains only shared fixtures and the
scoped environment guard.

Thư mục này chứa các owner hành vi của integration target
`native_engine_tests` do Cargo tự phát hiện. File target ở ngoài giữ các đăng
ký `#[tokio::test]` ổn định; mỗi owner chứa thân test tương ứng và fixture theo
protocol. `support/` chỉ giữ fixture dùng chung và guard environment có phạm vi.
