# Bare install mapping

Converts a bare install command and detected core name into the shared install request. The dispatch-level install mapping owner builds the final `CoreCommand`.

## Vietnamese

Folder này chuyển lệnh `mgc install` cùng core đã phát hiện thành request cài đặt dùng chung. `dispatch/install_mapping/` chịu trách nhiệm dựng `CoreCommand` cuối cùng; lỗi verb sai thuộc owner cục bộ này.
