# Cloud build owner / Owner build Cloud

## Responsibility / Trách nhiệm

Owns the existing dry-run Cloud build routes and their constrained environment/arguments: CDK synth, Pulumi preview and Terraform plan. A build route must not deploy cloud state or bypass MagiCore's process policy.

Sở hữu các route build dry-run Cloud hiện có cùng environment/arguments giới hạn: CDK synth, Pulumi preview và Terraform plan. Route build không được deploy state cloud hay bypass process policy của MGC.

## Errors and tests / Lỗi và test

The owner-local `error.rs` defines missing Cloud framework/toolchain and Cloudflare-to-CICD boundary diagnostics. CDK missing-local-executable remains in the shared error owner; dry-run commands and process policy do not change. Tests remain deferred until source closure.

`error.rs` cục bộ định nghĩa lỗi thiếu framework/toolchain Cloud và ranh giới Cloudflare thuộc CICD. Lỗi thiếu executable CDK vẫn ở shared error owner; lệnh dry-run và process policy không đổi. Test chờ đến khi đóng source pass.