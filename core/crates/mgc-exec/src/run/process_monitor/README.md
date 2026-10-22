# Process monitor

Owns the state carried across spawn, platform scanners and supervision. `types/` defines monitor and audit contexts; `compat_audit/` records compatible package-manager children once with argument redaction and typed failures. Platform detection/termination remain in `process_scan/`, `windows_job/` and `termination/` so this owner holds shared context and audit policy only.

# Process monitor

Sở hữu trạng thái truyền giữa spawn, scanner từng nền tảng và supervision. `types/` định nghĩa monitor/audit context; `compat_audit/` ghi mỗi package manager child được cho phép một lần, có redaction argv và lỗi typed. Nhận diện/kết thúc process vẫn ở `process_scan/`, `windows_job/` và `termination/`; owner này chỉ giữ context dùng chung và chính sách audit.

`ProcessMonitor` carries the effective root `ExecutionScope` into platform descendant scanners. The scanner applies the existing `adb`/`xcrun` DeviceControl restriction consistently on Unix and Windows; this process monitor is not a general sandbox and does not classify every device-control alias or descendant runtime.

`ProcessMonitor` truyền `ExecutionScope` hiệu lực của root vào scanner descendant theo OS. Scanner áp rule DeviceControl hiện có cho `adb`/`xcrun` trên Unix và Windows; monitor này không phải sandbox tổng quát và chưa phân loại mọi alias thiết bị hay runtime descendant.

`tracking/` owns the Unix process-tree environment marker and per-process token generator shared by spawn and descendant observation; it is an internal correlation token, not a security credential.

`tracking/` sở hữu marker biến môi trường cây process Unix và bộ sinh token trong process dùng chung bởi spawn và quan sát descendant; đây là token đối chiếu nội bộ, không phải credential bảo mật.
