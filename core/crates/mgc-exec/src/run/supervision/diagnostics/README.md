# Supervision diagnostics

Owns typed timeout and forbidden-descendant diagnostics. `mod.rs` exports the stable supervision entry points; `diagnostics.rs` captures bounded tails and native audit outcomes; `error.rs` defines typed timeout/child errors and retains audit-write causes. It labels canonical `adb`/`xcrun` names as device-control tools, including Windows root-exit reports that only retain the child name, while preserving existing wording for other tools. `test/mod.rs` explicitly registers diagnostic-construction and error-contract tests; `docs/` tracks the owner.

# Chẩn đoán supervision

Sở hữu chẩn đoán timeout và descendant bị cấm có kiểu. `mod.rs` export entry supervision ổn định; `diagnostics.rs` gom tail có giới hạn và trạng thái audit native; `error.rs` định nghĩa lỗi timeout/child typed, giữ source lỗi ghi audit. Tên canonical `adb`/`xcrun` được gọi là device-control tool, kể cả report Windows root-exit chỉ còn giữ tên child; cách gọi hiện tại của các tool khác được giữ. `test/mod.rs` đăng ký tường minh các test tạo diagnostic và hợp đồng lỗi; `docs/` theo dõi owner.
