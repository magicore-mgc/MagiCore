# Project per-core settings owner

This owner holds the serialized settings DTOs and scaffold-default mapping for the individual MagiCore cores: AI, App, CICD, Cloud, Game, IoT, and Lib. Each type and its `for_scaffold` constructor stay together in a same-name source file. The adjacent `project/scaffold_defaults/` coordinator owns shared project fields and dispatches to these per-core constructors; the public `ProjectConfig` path and its `from_scaffold` signature stay stable.

Field names, serde defaults, omission rules, derives, and scaffold values are compatibility contracts. A field or default change needs an approved design and focused schema migration review. Generic `DedupeConfig` remains outside this owner. These structs do not validate or execute user input, so deserialization and domain failures remain with the project loader and there is no invented local `error.rs`.

Nearest serde and per-core scaffold contract tests live under `test/`, with each core's cases beside its DTO. `mgc-config/tests/project.rs` keeps broad public-API and TOML-loading integration coverage. The source-first extraction moves the 16 per-core cases without changing their test items; none are executed in this phase.

# Owner cấu hình riêng từng core của project

Owner này chứa DTO cấu hình tuần tự hóa và mapping default scaffold của từng core MagiCore: AI, App, CICD, Cloud, Game, IoT và Lib. Mỗi type cùng constructor `for_scaffold` nằm trong file cùng tên. Coordinator kế bên `project/scaffold_defaults/` sở hữu field project chung và dispatch đến constructor từng core; public path `ProjectConfig` và chữ ký `from_scaffold` vẫn ổn định.

Tên field, serde default, quy tắc bỏ field rỗng, derive và giá trị scaffold là contract tương thích. Thay đổi chúng cần thiết kế được duyệt và rà soát migration schema. `DedupeConfig` dùng chung vẫn nằm ngoài owner này. Các struct không tự validate hay chạy input người dùng, vì vậy lỗi deserialize/domain do project loader sở hữu; không tạo `error.rs` giả.

Test serde và contract scaffold từng core nằm gần owner trong `test/`, mỗi file chứa case của DTO cùng tên. `mgc-config/tests/project.rs` giữ integration test tổng quát cho public API và TOML loading. Lát source-first chuyển 16 case từng core mà không đổi test item; phase này chưa chạy test nào.
