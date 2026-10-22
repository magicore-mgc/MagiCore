# v3_to_v4 / v3_to_v4

Converts legacy package pins into the explicit v4 graph. Registry identity, package projection and artifact integrity have separate owners. Chuyển pin package cũ thành đồ thị v4 tường minh. Registry, phép chiếu package và integrity artifact có owner riêng.

`v3_to_v4.rs` coordinates source-catalog and package-conversion owners while preserving warning order. `mod.rs` only declares/re-exports modules and registers tests.

`v3_to_v4.rs` điều phối owner catalog nguồn và chuyển đổi package, giữ thứ tự warning. `mod.rs` chỉ khai báo/re-export module và đăng ký test.
