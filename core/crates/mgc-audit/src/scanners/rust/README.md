# Native Rust OSV scanner

EN: This owner exposes the stable Rust audit and Cargo.lock pin-reading routes. Production audit uses the shared bounded project reader and MGC's OSV client; it does not spawn `cargo-audit`.

VI: Owner này export route audit Rust và đọc pin Cargo.lock ổn định. Audit production dùng bộ đọc project có giới hạn và OSV client của MGC; không spawn `cargo-audit`.
