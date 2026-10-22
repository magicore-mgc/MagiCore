# Unix active scan

Owns root-visibility retries and fail-closed classification while the Unix command is still running.

The Unix-only five-attempt root-visibility limit is owned here. The 20 ms poll interval stays in the shared runner because the wait loop also consumes it. Child-status checks, monitored PID set, process-group evidence, root-reaped state, audit context, and `MonitoredRootNotVisible` handling keep their existing order.

# Unix scan khi process đang chạy

> Owner này quản lý retry khi không thấy root và phân loại fail-closed lúc command Unix đang chạy.

> Giới hạn năm lần retry root-visibility chỉ thuộc owner Unix này. Khoảng poll 20 ms vẫn ở runner dùng chung vì wait loop cũng sử dụng. Giữ nguyên thứ tự kiểm tra child status, PID set, process-group evidence, trạng thái root đã reap, audit context và xử lý `MonitoredRootNotVisible`.
