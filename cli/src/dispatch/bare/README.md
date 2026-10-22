# Bare package routing

Owns auto-detected package operations for projects without an explicit core-specific command.

## Vietnamese

Folder này xử lý install/add/remove/update/list theo core đã chỉ định hoặc phát hiện từ danh tính dự án. Mỗi thao tác có mapper riêng và giữ nguyên các cổng từ chối hiện tại. `add/`, `install/` và `list/` chuẩn hóa request rồi dùng owner chung dựng command cho đủ chín core.
`remove/` normalizes the detected core and delegates command construction to the shared `dispatch/remove_mapping/` owner. Its local error covers only a wrong-verb call into the mapper.

`remove/` chuẩn hóa core đã nhận diện rồi giao việc dựng lệnh cho owner dùng chung `dispatch/remove_mapping/`. Lỗi cục bộ chỉ xử lý trường hợp gọi nhầm verb vào mapper.
`update/` resolves the detected core and normalizes bare update arguments before delegating to `dispatch/update_mapping/`. Its local error protects only against a wrong-verb mapper call.

`update/` phân giải core đã nhận diện, chuẩn hóa argument update bare rồi gọi `dispatch/update_mapping/`. Lỗi cục bộ chỉ chặn trường hợp gọi nhầm verb vào mapper.


The shared `ecosystem/` resolver and `validate/` preflight are documented, tested owners. Their existing error category remains shared in `error.rs` because it serves all bare operation mappers.

Resolver `ecosystem/` và preflight `validate/` dùng chung nay có owner được ghi tài liệu và test riêng. Nhóm lỗi hiện có tiếp tục dùng chung trong `error.rs` vì phục vụ mọi mapper bare.
