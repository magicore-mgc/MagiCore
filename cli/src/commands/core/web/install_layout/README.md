# Web install-layout policy

This owner reads `MAGICORE_WEB_STRICT_LAYOUT`. Values `0`, `false`, `no`, `off`, `legacy`, and `flat` select the legacy flat layout; `1`, `true`, `yes`, `on`, unknown values, and an unset variable select strict layout. Parsing trims whitespace and ignores ASCII case. Non-Web cores always return false. `core::shared::should_use_legacy_flat_layout` remains as a compatibility export.

The policy is total and has no error path, so this owner has no artificial `error.rs`. Pure parsing tests avoid mutating process-global environment state. No test or build has been run during this structural pass.

# Policy layout install Web

Owner này đọc `MAGICORE_WEB_STRICT_LAYOUT`. Giá trị `0`, `false`, `no`, `off`, `legacy`, `flat` chọn layout phẳng cũ; `1`, `true`, `yes`, `on`, giá trị lạ và biến chưa đặt chọn layout strict. Parser trim khoảng trắng và không phân biệt hoa thường ASCII. Core khác Web luôn trả false. `core::shared::should_use_legacy_flat_layout` được giữ làm facade tương thích.

Policy luôn trả kết quả và không có nhánh lỗi, nên không tạo `error.rs` giả. Test parser thuần không sửa environment toàn process. Lượt cấu trúc này chưa chạy test/build.
