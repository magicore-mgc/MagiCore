# cargo_source_repair

EN: Repairs legacy MagiCore Cargo stores that carry `.cargo-checksum.json` but no Cargo `.cargo-ok` readiness marker. A repair happens only after the checksum record, the cached `.crate` archive and the extracted tree re-verify — the same trust bar as `mgc install` — so a tampered or unmanaged directory never gains a marker. Used by the managed Rust build for `lib`/`iot` and by dev for Rust `web` backends; no package manager and no `cargo install` is invoked.

VI: Vá store Cargo legacy của MagiCore (có `.cargo-checksum.json` nhưng thiếu marker `.cargo-ok`). Chỉ vá sau khi checksum record, archive `.crate` cache và cây source đã giải nén được xác minh lại — cùng mức tin cậy như `mgc install` — nên thư mục bị sửa hoặc không do MagiCore tạo không bao giờ được gắn marker. Dùng cho build Rust managed của `lib`/`iot` và dev backend Rust của `web`; không gọi package manager, không `cargo install`.
