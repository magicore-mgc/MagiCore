# Node Runtime

Detect JavaScript package entrypoints, find a usable host Node executable from absolute PATH entries, and create a JSON-quoted temporary launcher using Node's main-entrypoint dispatcher. CommonJS keeps `require.main`; ESM keeps package-type detection and top-level await.
Nhận diện entrypoint JavaScript, tìm Node host qua PATH tuyệt đối và tạo launcher tạm có JSON quoting, gọi dispatcher entrypoint của Node. CommonJS giữ `require.main`; ESM giữ package type và top-level await.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `node_runtime.rs`: `node_runtime.rs` owns detection, host lookup and launcher creation; `error.rs` owns typed missing-host-Node failure. `mod.rs` exports the runtime helpers.
- `test/node_runtime.rs`: `test/node_runtime.rs` checks extensions, host lookup and quoted entrypoint paths; `test/error.rs` checks typed missing-runtime output.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`node_runtime.rs giữ nhận diện, tìm host và tạo launcher; error.rs giữ lỗi thiếu Node typed. mod.rs export helper.`

## Test and limits / Test và giới hạn

`test/node_runtime.rs` checks extensions, host lookup and quoted entrypoint paths; `test/error.rs` checks typed missing-runtime output. Node is a host runtime, not a package language or an alternative package manager; no bundled Node binary is included.

test/node_runtime.rs kiểm extension, host lookup, quote path; test/error.rs kiểm lỗi typed. Node là runtime host, không phải language hay PM thay thế; không nhúng binary Node.

Run the complete owner suite with host Node: `cargo test -p mgc --lib commands::package_runtime:: -- --include-ignored --test-threads=1`. The explicit Node runtime test covers `.cjs`, `.mjs`, both `.js` package types, quoted paths, argv and caller cwd without registry access. CI runs this test in the Node-equipped lifecycle lane.
Chạy suite owner đầy đủ với Node host theo lệnh trên. Test runtime tường minh kiểm `.cjs`, `.mjs`, hai package type của `.js`, quoting path, argv và cwd mà không dùng registry; CI chạy tại lane lifecycle có Node.
