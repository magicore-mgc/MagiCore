# Bare list mapping

Converts a bare list command and detected core name into the shared list request. The dispatch-level list mapping owner builds the final `CoreCommand`.

## Vietnamese

Folder này đổi lệnh `mgc list` cùng tên core đã phát hiện thành request list dùng chung. `dispatch/list_mapping/` dựng `CoreCommand` cuối; lỗi verb sai thuộc owner cục bộ.
