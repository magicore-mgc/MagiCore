# configured_store_min_age

## Responsibility / Trách nhiệm
Loads the Web minimum-age setting from `mgc.toml`, converts project hours to seconds, and falls back to the existing release-policy database whose values are already seconds. It also carries the explicit missing-timestamp policy.

Nạp minimum age Web từ `mgc.toml`, đổi giờ project sang giây, rồi fallback về database release policy vốn dùng giây. Owner cũng giữ policy cho phép thiếu timestamp.

## Contract / Hợp đồng
This owner only reads configured values; it never invents a default. The coordinator (`supply_chain.rs`) applies `DEFAULT_MIN_RELEASE_AGE_HOURS` when both the project and the store name no age and the project has no `mgc.lock` yet (first resolution).

Owner này chỉ đọc giá trị đã cấu hình, không tự tạo default. Coordinator (`supply_chain.rs`) áp `DEFAULT_MIN_RELEASE_AGE_HOURS` khi cả project lẫn store không đặt tuổi và project chưa có `mgc.lock` (resolve lần đầu).

Project configuration has precedence. A present malformed/unsafe project config returns a typed error. When no project age override exists and the store fallback is needed, an unreadable existing database returns a typed error; an unused store is not opened when a project override already decides the policy. A missing database is treated as no database override and is never created by this reader. Project hours and store seconds are range-checked before reaching the resolver guard.

Project config có precedence. Config hiện diện nhưng sai/không an toàn trả lỗi typed. Khi không có age override trong project và cần fallback sang store, database có sẵn nhưng không đọc được sẽ trả lỗi typed; store không dùng tới sẽ không bị mở nếu project override đã quyết định policy. Database không có được xem là không có override và reader không tự tạo DB. Giá trị giờ của project và giây của store được kiểm tra phạm vi trước khi tới guard resolver.

## Tests / Kiểm thử
Nearest tests cover hour conversion, project precedence, explicit missing-time policy, malformed config/database, missing-DB no-create, symlink rejection and overflow. NOT_RUN.
