# Package cache concurrency target / Target concurrency package cache

This directory owns the `package_cache_concurrency.rs` Cargo integration target. The root target keeps the stable test harness route; `test/` groups each behavior in a same-name folder with its own wiring module and README.

Thư mục này sở hữu Cargo integration target `package_cache_concurrency.rs`. File root giữ đường harness ổn định; `test/` nhóm từng behavior trong folder cùng tên, có wiring module và README riêng.

The target covers simultaneous atomic writers and conflicting payload rejection. Runtime verification is deferred to the later test phase.

Target kiểm tra atomic writer chạy đồng thời và từ chối payload xung đột. Xác minh runtime để dành cho phase test sau.
