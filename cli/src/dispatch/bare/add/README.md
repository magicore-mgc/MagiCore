# Bare add mapping

Converts a bare add command and detected core name into the normalized shared add request. The dispatch-level add mapping owner builds the final `CoreCommand`.

## Vietnamese

Folder này đổi lệnh `mgc add` cùng core đã phát hiện thành request add dùng chung. `dispatch/add_mapping/` dựng `CoreCommand` cuối cùng; owner cục bộ xử lý core chưa nhận diện và verb sai.
