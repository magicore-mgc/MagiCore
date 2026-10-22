Platform-specific pre-spawn process isolation and monitored-inspector availability. The `mod.rs` file exports declarations; `process_isolation.rs` configures the child; `test/` covers the local contract.

Folder này cấu hình cô lập process trước spawn theo nền tảng và kiểm tra khả năng inspector. `mod.rs` export khai báo; `process_isolation.rs` cấu hình child; `error.rs` sở hữu lỗi typed khi inspector không khả dụng; `test/` kiểm tra hợp đồng tại chỗ.
