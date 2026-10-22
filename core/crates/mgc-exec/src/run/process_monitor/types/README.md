# Process monitor data types

Owns the shared records passed between spawn, descendant scanners, Windows Job inspection and supervision. `ProcessMonitor` carries the effective root `ExecutionScope` alongside enabled/exemption state and platform scanner inputs; that scope lets the scanner enforce the existing DeviceControl-only `adb`/`xcrun` rule on descendants. `CompatAuditContext` carries the compatibility scope and dedup state; `NativeAuditContext` carries the root invocation used by denial auditing. The structs are data contracts; audit behavior/errors live in the sibling `compat_audit/` owner.

# Type dữ liệu process monitor

Sở hữu các bản ghi dùng chung giữa spawn, scanner descendant, Windows Job inspection và supervision. `ProcessMonitor` chứa `ExecutionScope` hiệu lực của root cùng trạng thái bật/exemption và dữ liệu scanner theo nền tảng; scope này giúp scanner áp rule `adb`/`xcrun` chỉ dành cho DeviceControl lên descendant. `CompatAuditContext` chứa phạm vi compatibility cùng trạng thái chống trùng; `NativeAuditContext` chứa invocation gốc dùng cho denial audit. Struct là hợp đồng dữ liệu; hành vi/lỗi audit nằm ở owner `compat_audit/` kế bên.
