# V4 writer owner / Owner ghi v4

EN: Builds and merges a v4 lane, preserves sibling-owner rows, refuses signed mutation without authorization, and keeps matching replay material stable.
VI: Dựng/merge lane v4, giữ row owner anh em, chặn sửa lock đã ký khi chưa được phép và giữ nguyên material replay khớp.

Preserve signature checks, warning text, replay decisions and finalize-before-atomic-write ordering.
