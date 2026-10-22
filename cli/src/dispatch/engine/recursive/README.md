# Recursive workspace execution

This owner coordinates monorepo workspace discovery, filtering, dependency-first ordering, identity validation, and command dispatch. The coordinator validates all selected workspace identities before calling a package handler, so a later core mismatch cannot follow an earlier workspace mutation.

`workspace_filter/` owns package-name and relative-path selection. `workspace_order/` applies the shared dependency-first graph contract. `execution_plan/` resolves each workspace core before execution. `recursive.rs` owns the sequence and existing per-workspace error aggregation; `mod.rs` only wires modules and the engine export. Each helper and the cross-owner preparation contract has nearest tests.

## Tiếng Việt

Owner này điều phối khám phá monorepo, lọc workspace, sắp thứ tự dependency-first, xác minh identity và dispatch command. Coordinator xác minh identity của toàn bộ workspace đã chọn trước khi gọi package handler, tránh trường hợp core của workspace sau sai nhưng workspace trước đã bị thay đổi.

`workspace_filter/` xử lý lọc theo tên package và đường dẫn tương đối. `workspace_order/` áp dụng contract dependency-first dùng chung. `execution_plan/` resolve core từng workspace trước khi chạy. `recursive.rs` giữ thứ tự điều phối và cơ chế gom lỗi từng workspace; `mod.rs` chỉ nối module và export cho engine. Mỗi helper và contract chuẩn bị chung có test gần owner.
