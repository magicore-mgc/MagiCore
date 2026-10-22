# Remove lane dispatch

Dispatches package removal to the existing feature-enabled core lanes. The known-core predicate is shared with add; lane feature gates, argument order, `MCP_INSTALL_AFTER_REMOVE = false`, and error behavior are preserved. The baseline still has no hardware remove arm, so this structural move makes no claim that hardware removal is supported.

## Tiếng Việt

Điều phối xóa package tới lane core hiện có được bật feature. Predicate core dùng chung với add; giữ feature gate, thứ tự đối số, `MCP_INSTALL_AFTER_REMOVE = false` và hành vi lỗi. Baseline chưa có arm remove hardware; lần chuyển cấu trúc này không claim hardware remove được hỗ trợ.
