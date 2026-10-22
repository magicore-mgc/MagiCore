# Framework build script / Script build framework

Owns package-script discovery and execution orchestration. `chain/` parses and validates every chained command before execution; `script/` maps only supported framework-local launch shapes; `policy/` rejects package-manager delegation outside an explicitly qualified compatibility runner.

Sở hữu dò package script và điều phối thực thi. `chain/` parse/xác nhận toàn bộ command chain trước khi chạy; `script/` chỉ map launch shape framework-local đã hỗ trợ; `policy/` từ chối ủy quyền package manager ngoài compatibility runner khai báo tường minh.

The local typed policy error preserves the current diagnostic. Parse/spawn errors from shared services propagate unchanged. No shell is introduced.

Error policy typed cục bộ giữ nguyên diagnostic hiện tại. Lỗi parse/spawn từ service dùng chung được propagate nguyên trạng. Không tạo shell.
