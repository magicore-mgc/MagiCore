# seeds

Ordered Web framework package seeds — dữ liệu package framework Web có thứ tự.
This owner defines the seed data consumed by MGC Web framework creation and manifest seeding.

`mod.rs` is the compatibility export. `seeds.rs` owns only the stable ordered aggregate, `types.rs` owns the shared data shapes, and `frameworks/` contains one configuration file for each supported seed key. The catalog does not resolve versions or call a registry; `None` and explicit version constraints keep their existing meaning in the resolver owner.

`mod.rs` giữ export tương thích. `seeds.rs` chỉ gom catalog theo thứ tự ổn định, `types.rs` giữ kiểu dữ liệu chung, còn `frameworks/` có một file cấu hình cho từng seed key được hỗ trợ. Catalog không resolve version và không gọi registry; ý nghĩa hiện tại của `None` và version constraint tường minh vẫn thuộc owner resolver.

The order is part of the existing selection contract and is pinned by `test/catalog.rs`. Add a framework by adding its data leaf, registering it in `frameworks/mod.rs`, then inserting its alias in the explicit order in `seeds.rs`. Keep package names, sections, TypeScript-only flags, and version constraints sourced from a supported framework contract; do not infer “latest” values here.

Thứ tự là một phần contract lựa chọn hiện tại và được khóa trong `test/catalog.rs`. Thêm framework bằng cách thêm file dữ liệu, đăng ký tại `frameworks/mod.rs`, rồi chèn alias vào đúng thứ tự trong `seeds.rs`. Package name, section, cờ TypeScript-only và version constraint phải có nguồn contract framework; không tự suy đoán version “latest” tại đây.

This owner has no local error module because its leaves are static data and do not fail independently. Its test folder covers catalog ordering and the framework-specific seed constraints that were previously tested at the command layer.

Owner này không có error module riêng vì các leaf chỉ là dữ liệu tĩnh và không tự phát sinh lỗi. Folder test kiểm tra thứ tự catalog cùng các constraint seed framework trước đây được kiểm tra ở tầng command.
