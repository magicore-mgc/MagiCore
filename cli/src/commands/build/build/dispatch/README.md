# Build dispatch / Điều phối build

## Responsibility / Trách nhiệm

`dispatch.rs` owns `mgc build` routing: validate compatibility flags, discover project root, load `ProjectContext`, select the core handler and report an unsupported route. It does not implement each core's compiler command.

`dispatch.rs` sở hữu điều phối `mgc build`: kiểm tra compatibility flags, tìm project root, nạp `ProjectContext`, chọn handler core và báo route không hỗ trợ. File không triển khai lệnh compiler riêng của từng core.

## Boundary / Ranh giới

Inputs are the CLI core/target/runtime options. Build-tool failures and project errors continue through the existing CLI diagnostics. Unit coverage currently lives in `cli/src/test/build_test.rs`; no test was run in this source-only pass.

Input là các tùy chọn core/target/runtime từ CLI. Lỗi tool build và project tiếp tục đi qua diagnostics hiện hành. Unit test hiện nằm tại `cli/src/test/build_test.rs`; batch source-only chưa chạy test.
