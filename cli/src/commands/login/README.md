# Login command

EN: Owns the shared `mgc login` route that obtains registry credentials, requests a user token, and stores that token in the selected `.npmrc`. The stable `commands::login::{run, LoginArgs, read_password, create_registry_user}` paths are re-exported here because the common dispatcher and registry user-add flow both depend on them.

VI: Folder này sở hữu route `mgc login` dùng chung: nhận credential, yêu cầu registry cấp token và lưu token vào `.npmrc` được chọn. Giữ các đường dẫn `commands::login::{run, LoginArgs, read_password, create_registry_user}` vì dispatcher chung và luồng thêm user registry cùng sử dụng.

## Owners and invariants

- `login/login.rs` coordinates the existing order: choose registry, obtain username/password, create the user, persist the token, then print one success line.
- `credentials/` owns visible username input and explicit stdin or hidden password input. Stdin remains bounded to 8 KiB and only trailing CR/LF is stripped.
- `registry_user_creation/` owns the registration request, URL policy mapping, bounded response read and token validation. It keeps the shared registry URL/client policy, HTTPS except loopback, redirects disabled, the existing timeout and the 32 KiB response limit.
- `token_storage/` owns global `~/.npmrc` versus project `.npmrc` selection and delegates scoped token persistence to `mgc-config`.
- `error.rs` preserves current English diagnostics with local typed categories and transparent sources where the old path retained them.

The public dispatcher and registry user-add caller remain on the stable facade. This is a source-structure change; it does not claim that registry authentication passed runtime or security verification. Tests have been authored or moved near owners and remain `NOT_RUN` under the source-first phase.

## Extension and verification

Add registry-specific request behavior under `registry_user_creation/`; keep URL validation delegated to the shared registry URL owner. Do not log passwords, tokens, request bodies or response bodies. Add tests beside the owner and preserve the existing `cli/tests/login_security_e2e.rs` route contract for the later verification phase.
