# Output capture

Owns stdout/stderr reader threads, channel delivery, bounded waiting, and `ExecOutcome` construction.

Reader threads continue draining inherited pipes so a child cannot block on a full OS pipe. `read_stream` retains the existing bounded suffix policy. Bounded drain deadlines remain the post-termination escape hatch; reader threads are not synchronously joined.

## Test ownership

Output read/disconnect and deadline cases belong in this owner’s `test/` during the later verification pass. The current source-only pass keeps existing supervision tests untouched.

# Thu output process con

> Owner này quản lý reader thread cho stdout/stderr, channel, deadline chờ và việc tạo `ExecOutcome`.

> Thread luôn drain pipe để child không bị chặn khi pipe đầy; `read_stream` vẫn giữ chính sách suffix có giới hạn. Drain sau termination vẫn có deadline và không `join` đồng bộ reader.

> Test cho lỗi đọc, reader mất kết nối và deadline sẽ thuộc owner này trong lượt kiểm thử sau.
