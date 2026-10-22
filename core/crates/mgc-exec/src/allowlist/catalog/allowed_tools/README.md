# Allowed executable catalog

Owns the reviewed passthrough executable list. Device and language scope rules remain in their policy owners.

# Allowed executable catalog / Vai trò

Sở hữu danh sách executable passthrough đã review; giới hạn scope nằm ở policy owner.

- Implementation / Implementation: `allowed_tools.rs` when present in this owner.
- Tests / Test: `test/`, registered by the owner `mod.rs` when behavior needs verification.
- Contract / Contract: preserve current public paths, error diagnostics and policy order; no behavior change is part of the structural move.

## Contract and safe extension

`mod.rs` wires/re-exports only. Keep implementation in the same-name source file and delegate distinct policy/parser/filesystem behavior to its named child owner. Preserve public paths, decision order, exact existing diagnostics and host-specific branches during structural work. Errors remain at the operation that detects them; do not add a new error type/file unless it expresses a real stable classification or recovery contract. Add each new behavior assertion to the nearest `test/` owner and update its README/test map.

The nearest crate verification command after the structural phase is `cargo test -p mgc-exec`; it has **not** been run for this structural pass.

## Contract và mở rộng an toàn

`mod.rs` chỉ wiring/re-export. Implementation nằm trong source file cùng tên; tách policy/parser/filesystem khác biệt sang child owner có tên rõ. Trong refactor cấu trúc, giữ public path, thứ tự quyết định, diagnostic hiện hành và nhánh riêng từng OS. Error thuộc operation phát hiện; chỉ tạo error type/file mới khi có phân loại/recovery ổn định thực sự. Behavior mới phải có assertion trong `test/` gần nhất và cập nhật README/test map.

Lệnh kiểm chứng crate sau phase cấu trúc: `cargo test -p mgc-exec`; **chưa chạy** ở batch này.
