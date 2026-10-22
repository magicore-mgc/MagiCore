# Tool authorization coordinator

Preserves the ordered authorization flow while delegating each independent deny/scope/membership decision to a focused owner.

# Tool authorization coordinator / Vai trò

Giữ thứ tự luồng cấp quyền và giao từng quyết định deny/scope/membership cho owner riêng.

- Implementation / Implementation: `tool_check.rs` when present in this owner.
- Tests / Test: `test/`, registered by the owner `mod.rs` when behavior needs verification.
- Contract / Contract: preserve current public paths, error diagnostics and policy order; no behavior change is part of the structural move.

## Contract and safe extension

`mod.rs` wires/re-exports only. Keep implementation in the same-name source file and delegate distinct policy/parser/filesystem behavior to its named child owner. Preserve public paths, decision order, exact existing diagnostics and host-specific branches during structural work. Errors remain at the operation that detects them; do not add a new error type/file unless it expresses a real stable classification or recovery contract. Add each new behavior assertion to the nearest `test/` owner and update its README/test map.

Concrete project/package binaries use `check_project_binary_scope`: it applies device-control and `PROJECT_LANGUAGE_EXECUTABLES` scope denies to the root executable without applying the static host-tool allowlist. This preserves caller-selected package binaries while preventing a direct root binary named `adb`/`xcrun` or a runtime in that catalog from bypassing its existing scope rule. The current catalog excludes Node, Python, Go and Cargo; the owner test explicitly pins Python/Cargo exclusion but does not pin Node/Go. S468 preserves this existing catalog; a broader language/runtime classification needs separate policy review. The gate does not inspect descendants launched by an allowed script. Package-manager, dependency-mutation and script checks remain in the project-binary owner. `ScopeConstraints` fields are descriptive declarations; execution does not consume `constraints()` to enforce cwd, network, audit or generic argv limits.

The nearest crate verification command after the structural phase is `cargo test -p mgc-exec`; it has **not** been run for this structural pass.

## Contract và mở rộng an toàn

`mod.rs` chỉ wiring/re-export. Implementation nằm trong source file cùng tên; tách policy/parser/filesystem khác biệt sang child owner có tên rõ. Trong refactor cấu trúc, giữ public path, thứ tự quyết định, diagnostic hiện hành và nhánh riêng từng OS. Error thuộc operation phát hiện; chỉ tạo error type/file mới khi có phân loại/recovery ổn định thực sự. Behavior mới phải có assertion trong `test/` gần nhất và cập nhật README/test map.

Binary project/package cụ thể đi qua `check_project_binary_scope`: áp deny device-control và `PROJECT_LANGUAGE_EXECUTABLES` cho executable gốc theo scope nhưng không áp allowlist tĩnh của tool host. Cách này giữ binary package do caller chọn, đồng thời ngăn binary gốc tên `adb`/`xcrun` hoặc runtime trong catalog bypass scope hiện có. Catalog hiện loại Node, Python, Go, Cargo; test owner chỉ khóa việc loại Python/Cargo, chưa khóa Node/Go. S468 giữ catalog hiện hành; phân loại ngôn ngữ/runtime rộng hơn cần review policy riêng. Gate chưa kiểm tra descendant do script được phép spawn. Guard package manager, mutation dependency và script vẫn thuộc owner project-binary. Các trường `ScopeConstraints` là khai báo mô tả; executor chưa dùng `constraints()` để ép cwd, network, audit hoặc giới hạn argv tổng quát.

Lệnh kiểm chứng crate sau phase cấu trúc: `cargo test -p mgc-exec`; **chưa chạy** ở batch này.
