# Timeout

Owns execution-deadline checks and fail-closed process-tree cleanup after the configured wait budget expires.

At root exit, the caller first drains for the remaining execution budget. If output is incomplete, this owner terminates the tree, tries the direct-child kill, performs the bounded post-kill drain, and returns the existing timeout diagnostic.

# Timeout

> Owner này kiểm tra deadline và dọn cây process fail-closed khi hết thời gian thực thi.

> Khi root đã thoát, caller drain theo thời gian còn lại trước; nếu output chưa hoàn tất, owner terminate cây, thử kill child, drain sau-kill có giới hạn và trả diagnostic timeout hiện tại.
