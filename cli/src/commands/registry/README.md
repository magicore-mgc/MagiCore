# Registry command owners

This folder exposes the stable `commands::registry` CLI API and routes requests to argument, trusted-binding, user-management, and registry-URL owners. Keep `mod.rs` limited to module wiring and public exports; command behavior belongs in `registry.rs` or its focused owner.

Thư mục này giữ API CLI `commands::registry` ổn định và nối lệnh tới các owner argument, trusted binding, quản lý user và URL registry. `mod.rs` chỉ wiring/export; hành vi lệnh nằm trong `registry.rs` hoặc owner chuyên trách.
