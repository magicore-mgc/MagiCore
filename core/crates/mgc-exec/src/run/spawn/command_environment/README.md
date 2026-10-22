# Child command environment

This owner applies the existing `ExecOptions` environment to a constructed child `Command`. It places MagiCore deny shims before the selected caller PATH, preserves explicit non-PATH values, and restores the same non-Unix critical variables after `env_clear`. It does not create the shadow directory, inspect processes, spawn a child, or own PATH/error policy; `run/environment/` remains the source for guarded PATH composition and `EnvironmentError`.

Owner này áp environment từ `ExecOptions` lên child `Command` đã dựng. Nó đặt MagiCore deny shim trước PATH caller được chọn, giữ giá trị explicit không phải PATH và khôi phục cùng các biến critical non-Unix sau `env_clear`. Owner không tạo shadow directory, scan process hoặc spawn child; cũng không sở hữu policy ghép PATH/lỗi. Nguồn chuẩn của guarded PATH và `EnvironmentError` vẫn là `run/environment/`.
