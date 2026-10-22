# Registry auth test owners / Owner test auth Registry

## Purpose / Mục đích

This directory owns test cases registered by the existing auth test module at src/test/auth.rs. The parent file remains the Rust test entry and keeps crate-auth imports plus the shared OIDC fixture. This folder contains only focused test owners and module wiring; production auth policy stays in src/auth/.

Thư mục này sở hữu test được đăng ký bởi auth test module hiện tại tại src/test/auth.rs. File cha vẫn là Rust test entry, giữ import của auth crate và fixture OIDC dùng chung. Thư mục này chỉ chứa test owner và module wiring; auth policy production ở src/auth/.

## Module map / Sơ đồ module

| File | Coverage |
| --- | --- |
| trusted_token_authorization.rs | Package-scoped npm/OCI access and trusted-token registry reads |
| trusted_transport.rs | Verified HTTPS, loopback binding and rejection on unverified HTTP |
| trusted_token_lifecycle.rs | Legacy-token denial, protocol namespace and expiration |
| scoped_account_policy.rs | Scoped package/global reads and anonymous-read defaults |
| scoped_account_routes.rs | HTTP route enforcement for package and collection reads |
| mod.rs | Private test module wiring and inherited shared fixture/imports |

All 12 test bodies and attributes are preserved. Runtime behavior has not been executed in this source-first phase.

Giữ nguyên body và attribute của đủ 12 test. Runtime chưa được chạy trong giai đoạn source-first này.
