# esp32-feature-e2e

Clean input fixture for the live IoT installer, optimizer, and build E2E.
This directory is not an installed sample project and intentionally contains
no generated lockfiles, package cache, `.magicore`, or optimizer output.

The live CI E2E copies this manifest, Cargo target configuration, core marker,
and source into a fresh temporary project. It runs the release `mgc` binary
through these product steps:

1. `mgc install-iot` resolves and verifies the ESP32-C3 Rust dependency graph.
   It writes canonical `mgc.lock` and the Cargo-compatible `Cargo.lock`.
2. `mgc optimizer` detects the Rust project and writes
   `.mgc-optimizer/rust_cargo_profile.toml` from the runner's hardware facts.
3. `mgc build` compiles for `riscv32imc-unknown-none-elf`, verifies that Cargo
   consumed the optimizer profile, and leaves the audited Cargo invocation in
   project-local `.magicore/exec.log`.

The package artifacts are user-global, shared data, not `.magicore` contents:

- `$MGC_DATA_DIR/store/cargo/registry/cache/` holds verified `.crate` archives.
- `$MGC_DATA_DIR/store/cargo/registry/src/` holds the Cargo-compatible source
  directories used by MGC-managed builds.
- `$MGC_DATA_DIR/store/v3/` is the MagiCore content-addressable store (CAS).

The live E2E gives each run a fresh project, HOME, and MagiCore data directory,
so pre-existing locks or caches cannot make it pass. A separate hermetic
release-binary E2E creates two independent Rust projects with one shared store,
requires exactly one archive download total, and rejects a tampered cached
archive. Together the tests show both a clean install/build and cross-project
reuse; the live fixture by itself does not claim to benchmark build speed.

## Bản tiếng Việt

Đây là fixture đầu vào sạch cho E2E IoT install, optimizer và build; không phải
project mẫu đã cài sẵn. Không commit lockfile, package cache, `.magicore` hay
output optimizer được tạo bởi lần chạy trước.

CI chép manifest, cấu hình target Cargo, core marker và source sang project tạm
mới, rồi dùng binary release thật để chạy:

1. `mgc install-iot` resolve và xác minh graph Rust ESP32-C3, ghi `mgc.lock`
   chuẩn của MagiCore cùng `Cargo.lock` tương thích Cargo.
2. `mgc optimizer` nhận diện Rust và sinh
   `.mgc-optimizer/rust_cargo_profile.toml` từ thông tin phần cứng runner.
3. `mgc build` biên dịch target `riscv32imc-unknown-none-elf`, xác nhận Cargo
   đã nạp profile optimizer và ghi lần gọi Cargo vào `.magicore/exec.log`.

Package dùng chung nằm trong data dir của user, không nằm trong `.magicore` của
từng project: archive ở `store/cargo/registry/cache`, source Cargo ở
`store/cargo/registry/src`, còn CAS ở `store/v3` bên dưới `$MGC_DATA_DIR`.

E2E live cô lập project, HOME và data dir để lock/cache cũ không thể làm test
pass. E2E hermetic riêng trên binary release tạo hai project độc lập dùng một
store, bắt buộc chỉ tải archive một lần và từ chối archive cache đã bị sửa.
Các test chứng minh luồng cài/build và tái sử dụng package; không tự nhận đã
đo benchmark hay chứng minh build nhanh hơn.
