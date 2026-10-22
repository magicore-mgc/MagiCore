# Execution scope and constraints

Owns the `ExecutionScope` variants and the descriptive `ScopeConstraints` defaults. The current executor does not consume `constraints()`; only explicit tool/invocation guards and caller options affect execution.

# Execution scope and constraints / Vai trò

Sở hữu các biến thể `ExecutionScope` và default `ScopeConstraints` mang tính mô tả. Executor hiện không dùng `constraints()`; chỉ các guard tool/invocation riêng và options của caller mới tác động luồng chạy.

- Implementation / Implementation: `scope.rs` when present in this owner.
- Tests / Test: `test/`, registered by the owner `mod.rs` when behavior needs verification.
- Contract / Contract: preserve current public paths, error diagnostics and policy order; no behavior change is part of the structural move.

## Contract and safe extension

`mod.rs` wires/re-exports only. Keep implementation in the same-name source file and delegate distinct policy/parser/filesystem behavior to its named child owner. `ScopeConstraints` is descriptive metadata, not an enforcement token: do not claim cwd lock, network isolation, audit-required behavior or a generic argv allowlist from these booleans. Preserve the public struct layout and add explicit enforcement only behind reviewed executor behavior and nearest-owner tests. Preserve public paths, decision order, exact diagnostics and host-specific branches. Errors remain at the operation that detects them; do not add a new error type/file unless it expresses a real stable classification or recovery contract.

The nearest crate verification command after the structural phase is `cargo test -p mgc-exec`; it has **not** been run for this structural pass.

## Contract và mở rộng an toàn

`mod.rs` chỉ wiring/re-export. Implementation nằm trong source file cùng tên; tách policy/parser/filesystem khác biệt sang child owner có tên rõ. `ScopeConstraints` chỉ là metadata mô tả, không phải token enforcement: không tuyên bố đã khóa cwd, cô lập mạng, bắt audit hay tạo allowlist argv chung dựa trên các boolean này. Giữ nguyên layout struct public; chỉ thêm enforcement qua behavior executor đã review và test gần owner. Trong refactor giữ public path, thứ tự quyết định, diagnostic hiện hành và nhánh riêng từng OS. Error thuộc operation phát hiện; chỉ tạo error type/file mới khi có phân loại/recovery ổn định thực sự.

Lệnh kiểm chứng crate sau phase cấu trúc: `cargo test -p mgc-exec`; **chưa chạy** ở batch này.
