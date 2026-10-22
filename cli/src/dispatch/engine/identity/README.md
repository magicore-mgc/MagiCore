# Engine identity

This owner reads workspace package names, resolves persisted project-core identity, maps explicit per-core dependency commands, and validates selector conflicts. It propagates malformed manifest errors; only a missing manifest produces no package name so the recursive caller can use its directory-name fallback. It does not dispatch commands or mutate workspace configuration.

`identity.rs` contains the implementation. `test/` keeps workspace identity, route mapping, and package-name tests beside the owner. `mod.rs` only wires exports used by `engine` and `recursive`.

## Vietnamese

Owner này đọc tên package trong workspace, xác định core đã lưu của dự án, ánh xạ lệnh dependency có tên core tường minh và kiểm tra selector mâu thuẫn. Owner truyền tiếp lỗi manifest hỏng; chỉ khi thiếu manifest mới trả về không có tên package để caller recursive dùng tên thư mục dự phòng. Owner không dispatch command và không sửa cấu hình workspace.

`identity.rs` chứa implementation. `test/` đặt test identity workspace, ánh xạ route và tên package cạnh owner. `mod.rs` chỉ nối export cho `engine` và `recursive`.
