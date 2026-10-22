# Spawn report owner

This owner converts the completed child status and bounded output into `ExecReport`, then applies the caller's allowed-exit-code policy. Audit persistence remains in the spawn coordinator and runs before exit-code enforcement.

Owner này chuyển trạng thái child và output đã giới hạn thành `ExecReport`, rồi áp dụng danh sách exit code được caller cho phép. Ghi audit vẫn do coordinator spawn quản lý và diễn ra trước khi áp chính sách exit code.
