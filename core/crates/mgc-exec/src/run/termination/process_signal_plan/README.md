# process_signal_plan / policy chọn signal

Owns the pure mapping from observed/live process groups and root-reaped state to deduplicated process-group targets plus the direct-root fallback. `ProcessSignalPlan` stays private to this behavior owner; planning and type value tests are registered in `test/`.

Sở hữu phép ánh xạ thuần từ group đã thấy/đang sống và trạng thái root-reaped sang tập process group không trùng cùng fallback signal trực tiếp root. `ProcessSignalPlan` chỉ thuộc owner hành vi này; test planner/type đăng ký trong `test/`.
