# Shared JS manifest / Manifest JS dùng chung

This module owns `package.json` parsing, serialization, atomic writes, workspace protocol policy, and the stable `mgc_js_service::manifest` API.

Module này sở hữu việc đọc/ghi `package.json`, atomic write, policy workspace protocol và API ổn định `mgc_js_service::manifest`.

`mod.rs` is only the compatibility facade. Behavior lives in the same-name owners below. Web and App adapters depend on this service; they must not fork or bypass these checks.

`mod.rs` chỉ là facade tương thích. Hành vi nằm trong các owner cùng tên bên dưới. Adapter Web và App dùng service này, không được fork hoặc bỏ qua các kiểm tra.

The manifest owner preserves registry ranges, skips workspace ranges from the package graph, and refuses silent registry replacement of a workspace-owned dependency. Atomic-write owners reject linked/non-regular destinations and publish missing files through a same-directory staging file.

Owner manifest giữ nguyên range registry, bỏ workspace range khỏi package graph và từ chối thay dependency workspace bằng registry một cách im lặng. Owner atomic-write từ chối đích là symlink/non-regular và publish file mới bằng staging cùng thư mục.

Nearest behavior tests live in this module's owner `test/` folders. `core/crates/mgc-js-service/tests/manifest.rs` remains as the external public-route integration proof.

Test hành vi gần owner nằm trong `test/` của từng module. `core/crates/mgc-js-service/tests/manifest.rs` được giữ làm integration proof cho public route.
