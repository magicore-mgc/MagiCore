# Solve-local types / Kiểu riêng của solve

This owner contains types used only by the shared resolver solve flow: selected package tuples, prefetched version/dependency maps, and queued requests carrying an optional root-manifest index.

Owner này chứa type chỉ dùng trong luồng solve resolver: tuple package đã chọn, map version/dependency tải trước và request queue có thể mang index manifest root.

Queue provenance remains private to the resolver solve module tree. Tests remain near their owners or at current Cargo integration targets and are deferred until the source-first phase closes.
