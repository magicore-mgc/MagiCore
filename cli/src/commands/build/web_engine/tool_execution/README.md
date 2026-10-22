# Allowlisted tool execution / Thực thi tool qua allowlist

Creates the MagiCore `BuildRunner` execution options and delegates to the shared `mgc-exec` boundary. This owner does not spawn directly or alter argument/environment policy.

Dựng execution options `BuildRunner` của MagiCore rồi ủy quyền qua boundary `mgc-exec` dùng chung. Owner này không tự spawn hoặc đổi policy argument/environment.

Errors are owned by the shared executor and propagate unchanged; a duplicate local error type would obscure its source.

Lỗi thuộc executor dùng chung và được propagate nguyên trạng; error type trùng lặp sẽ che source.
