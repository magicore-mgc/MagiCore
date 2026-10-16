# `mgc-lockfile` — Engine lockfile thống nhất

Crate này cung cấp định dạng `mgc.lock` thống nhất, metadata kiểm tra toàn vẹn và khả năng import lockfile từ các package manager khác (npm, pnpm, yarn và bun).

## Khả năng

1. **Serialize và deserialize:** đọc, ghi biểu diễn TOML và JSON của `mgc.lock`.
2. **Kiểm tra checksum:** tạo và xác minh `mgc.lock.sha256`.
3. **Chữ ký BLAKE3 có khóa:** xác thực lockfile khi cấu hình `MAGICORE_LOCKFILE_KEY`.
4. **Import giữa các package manager:** nhập `package-lock.json` (npm v2/v3), pnpm lockfile có map `packages` (đã kiểm thử đầu ra v9.0 của pnpm 12.12.0), Yarn Classic v1 và Bun lockfile dạng text (đã kiểm thử lockfile version 1 và 2). Chưa hỗ trợ Bun `bun.lockb` dạng nhị phân và Yarn Berry. Import giữ pin đã resolve cùng integrity khi định dạng nguồn có dữ liệu đó; cạnh dependency của Bun chỉ giữ tên vì lockfile lưu range tại vị trí này.

## Cách dùng

### Đọc và xác minh lockfile

```rust
use std::path::Path;
use mgc_lockfile::read_lockfile_checked;

let project_root = Path::new("./my-project");
if let Some(lockfile) = read_lockfile_checked(project_root)? {
    println!("Core: {}, Packages: {}", lockfile.core, lockfile.packages.len());
}
```

### Ghi lockfile kèm checksum

```rust
use std::path::Path;
use mgc_lockfile::{Lockfile, write_lockfile};

let lockfile = Lockfile::new("web", "frontend");
write_lockfile(Path::new("./my-project"), &lockfile)?;
```

### Import lockfile cũ

```rust
use std::path::Path;
use mgc_lockfile::import::import_legacy_lockfile_explicit;
use mgc_types::Manifest;

let project_root = Path::new("./legacy-project");
let manifest = Manifest::new("app", mgc_types::Ecosystem::Web);
if let Some(migrated_lock) = import_legacy_lockfile_explicit(
    project_root, "web", "frontend", &manifest
)? {
    println!("Đã chuyển đổi thành công {} package!", migrated_lock.packages.len());
}
```

## Chạy test

Chạy unit test và integration test của crate bằng lệnh:

```bash
cargo test -p mgc-lockfile
```

Xem [`README.md`](README.md) để đọc hướng dẫn tiếng Anh.
