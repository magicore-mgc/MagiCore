# CLI dispatch

This folder maps parsed commands to shared handlers or one of MagiCore's core-specific handlers. `types/` owns the stable dispatch contracts and ecosystem detector; `per_core/` adapts explicit command schemas; `bare/` resolves project identity for generic package verbs; `add_mapping/`, `install_mapping/` and `list_mapping/` build shared nine-core package contracts; `engine/` owns global policy and workspace orchestration; `common/` owns the shared command router; `core/` executes core-specific routes.

## Vietnamese

Folder này ánh xạ lệnh đã parse sang handler dùng chung hoặc handler core. `types/` sở hữu hợp đồng dispatch ổn định và bộ dò ecosystem; `per_core/` chuyển schema lệnh tường minh; `bare/` tìm identity dự án cho lệnh package chung; `add_mapping/`, `install_mapping/` và `list_mapping/` dựng hợp đồng package dùng chung cho chín core; `engine/` điều phối policy và workspace; `common/` cùng `core/` thực thi route đã chọn.
The `remove_mapping/` owner builds the nine core-specific remove commands for both bare and explicit routes. It preserves Web's reinstall flag projection and forwards package/compatibility inputs; it owns no policy or runtime decisions.

Owner `remove_mapping/` dựng chín lệnh remove theo core cho cả route bare lẫn route tường minh. Owner giữ cách chiếu cờ cài lại riêng của Web và chuyển tiếp package/compat; không quyết định policy hay runtime.
The `update_mapping/` owner constructs all nine core-specific update commands for bare and explicit routes, forwarding package, install and compatibility fields without applying policy.

Owner `update_mapping/` dựng lệnh update cho đủ chín core ở route bare và tường minh, chuyển tiếp package, install, compat mà không áp dụng policy.
