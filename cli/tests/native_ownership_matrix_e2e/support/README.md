# E2E support / Hỗ trợ E2E

`cli.rs` selects and invokes MagiCore with a bounded timeout; `sandbox.rs` creates isolated homes and process-spawn canaries; `projects/` builds minimal manifests and project trees; `lifecycle/` assembles add/install/remove/update scenarios; `registries/` owns hermetic local package registries.

`cli.rs` chọn và gọi MagiCore có timeout; `sandbox.rs` tạo HOME cô lập và canary spawn; `projects/` dựng manifest/cây dự án tối thiểu; `lifecycle/` dựng kịch bản add/install/remove/update; `registries/` sở hữu registry package hermetic cục bộ.

Các helper được gọi bởi integration cases trong `../test/`; không có fallback gọi package manager ngoài. Không có kiểu lỗi riêng mới nên không tạo `error/`.
