# Install Policy

Construct native MGC install arguments and copy only explicit registry/security/lock/trust policy from the caller project.
Tạo argument cài native MGC và chỉ chép policy registry/security/lock/trust caller đã cấu hình.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `install_policy.rs`: `install_policy.rs` owns `registry_install_args` and `registry_project_config`; `error.rs` owns typed caller-policy failure. `mod.rs` exports the helpers and package-runner identity constants.
- `test/install_policy.rs`: `test/install_policy.rs` checks script opt-in, inherited policy, redaction and native defaults; `test/error.rs` checks the typed failure.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`install_policy.rs sở hữu registry_install_args và registry_project_config; error.rs giữ typed error policy caller. mod.rs export helper và identity project runner.`

## Test and limits / Test và giới hạn

`test/install_policy.rs` checks script opt-in, inherited policy, redaction and native defaults; `test/error.rs` checks the typed failure. Does not import caller compatibility execution settings. The installer still runs through MGC.

test/install_policy.rs kiểm script opt-in, policy kế thừa, redaction và default; test/error.rs kiểm lỗi typed. Không kế thừa compat execution; installer vẫn chạy qua MGC.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
