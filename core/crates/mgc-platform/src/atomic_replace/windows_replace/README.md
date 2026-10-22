# Windows replacement / Thay file Windows

This owner converts paths to NUL-terminated UTF-16, resolves only parent directories for extended paths, and calls `MoveFileExW` with replace-existing plus caller-selected write-through. It maps OS errors to `AtomicReplaceError` and moves a staging link itself rather than resolving its target.

Owner này chuyển path sang UTF-16 kết thúc NUL, chỉ resolve parent để tạo extended path và gọi `MoveFileExW` với cờ replace-existing cùng write-through do caller chọn. Owner chuyển OS error sang `AtomicReplaceError`, và thao tác di chuyển chính staging link, không resolve target.
