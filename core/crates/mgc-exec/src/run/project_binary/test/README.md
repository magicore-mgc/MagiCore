# Tests for `project_binary`

Local tests for project-binary entry points. Keep rejection checks side-effect-free: invalid paths and binaries outside their execution scope must fail before spawning a child process.

Test cục bộ cho entry point project binary. Giữ case bị từ chối không tạo side effect: path sai và binary ngoài scope phải dừng trước khi spawn process con.

`mod.rs` registers `project_binary.rs` as a named child module. Tests call the stable `crate::run` API and verify invalid paths and DeviceControl-only binaries are rejected before child creation.
`mod.rs` đăng ký `project_binary.rs` thành module con có tên. Test gọi API ổn định `crate::run`, kiểm tra path sai và binary chỉ dành cho DeviceControl bị từ chối trước khi tạo process con.
