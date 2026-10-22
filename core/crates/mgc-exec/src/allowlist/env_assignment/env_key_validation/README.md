# Environment key grammar validation

Owns the portable ASCII identifier grammar used to recognize leading environment assignments.

# Environment key grammar validation / Vai trò

Sở hữu grammar identifier ASCII dùng để nhận diện assignment biến môi trường.

- Implementation: `env_key_validation.rs`.
- Tests: `test/`, registered from the owner facade.
- Contract: preserve the current accepted input/output and diagnostics; no behavior change is part of the structural move.

## Contract and safe extension

`mod.rs` wires/re-exports only. Keep implementation in the same-name source file and delegate distinct policy/parser/filesystem behavior to its named child owner. Preserve public paths, decision order, exact existing diagnostics and host-specific branches during structural work. Errors remain at the operation that detects them; do not add a new error type/file unless it expresses a real stable classification or recovery contract. Add each new behavior assertion to the nearest `test/` owner and update its README/test map.

The nearest crate verification command after the structural phase is `cargo test -p mgc-exec`; it has **not** been run for this structural pass.

## Contract và mở rộng an toàn

`mod.rs` chỉ wiring/re-export. Implementation nằm trong source file cùng tên; tách policy/parser/filesystem khác biệt sang child owner có tên rõ. Trong refactor cấu trúc, giữ public path, thứ tự quyết định, diagnostic hiện hành và nhánh riêng từng OS. Error thuộc operation phát hiện; chỉ tạo error type/file mới khi có phân loại/recovery ổn định thực sự. Behavior mới phải có assertion trong `test/` gần nhất và cập nhật README/test map.

Lệnh kiểm chứng crate sau phase cấu trúc: `cargo test -p mgc-exec`; **chưa chạy** ở batch này.
