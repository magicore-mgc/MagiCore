# Build command owner map / Sơ đồ owner lệnh build

`mod.rs` is the CLI compatibility facade. `build.rs` is now a thin wiring layer; the all-core route lives under `build/` in same-name implementation modules. See [build owners](build/README.md) for responsibility, calls, limits and deferred error/test migration.

`mod.rs` là facade tương thích của CLI. `build.rs` hiện chỉ wiring; route build toàn core nằm dưới `build/` trong các module implementation cùng tên. Xem [các owner build](build/README.md) để biết trách nhiệm, luồng gọi, giới hạn và phần error/test migration còn mở.

The tree covers dispatch, shared project/tool lookup/Go environment, Web-Go output safety, AI, Game, IoT, Lib, Hardware, App configuration/multi-target and Cloud. The Web framework engine now lives under `web_engine/`, with focused target, script, Node, entrypoint, native-engine, executor, runtime and Rust-build owners.

Cây module bao gồm dispatch, project/tool lookup/Go environment dùng chung, output Web-Go an toàn, AI, Game, IoT, Lib, Hardware, cấu hình/multi-target App và Cloud. Engine framework Web hiện nằm trong folder `web_engine/` với các owner chức năng riêng.

The source move preserves shared diagnostics where they cross core boundaries. Owner-local typed errors now cover Web engine, App configuration/multi-target, AI, Game, IoT, Lib, Hardware, Cloud and Web-Go lanes; shared missing-local-executable and freshness errors remain in shared owners. Existing unit and E2E files retain their S165-start fingerprints; their pre-existing worktree diffs were not changed here. Tests, compilation, formatter and CLI execution were not run in this batch.

Lượt tách source giữ diagnostic dùng chung ở boundary liên core. Error typed cục bộ hiện bao phủ Web engine, App configuration/multi-target, AI, Game, IoT, Lib, Hardware, Cloud và Web-Go; lỗi executable local/freshness dùng chung tiếp tục ở shared owner. Unit/E2E giữ fingerprint tại đầu S165; các thay đổi dirty tồn tại từ trước không bị sửa trong batch. Batch này không chạy test, compile, formatter hay CLI.
