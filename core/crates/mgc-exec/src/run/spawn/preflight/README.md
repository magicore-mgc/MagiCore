# Spawn preflight

This owner validates arguments, resolves timeout and working directory, redacts argv, and records an explicitly declared compatibility-runner scope before any child can start. `complete` builds the dry-run report and optional audit row. It never resolves the executable or starts a process. `PreparedInvocation` carries validated data to the spawn coordinator. Existing validation, timeout, compat parsing, audit and `SpawnError` owners remain authoritative.

Owner này xác thực đối số, phân giải timeout và working directory, redact argv, đồng thời ghi scope compat-runner đã khai báo trước khi process con có thể chạy. `complete` tạo report dry-run và audit tùy chọn; owner không resolve executable và không spawn process. `PreparedInvocation` chuyển dữ liệu đã xác thực về coordinator. Các owner validation, timeout, compat parser, audit và `SpawnError` hiện hữu tiếp tục là nguồn chuẩn.
