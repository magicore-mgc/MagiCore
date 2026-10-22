# Safe clear / Clear an toàn

Refuses missing non-directory and symlink roots, then clears each direct child. Symlink/junction entries are removed as links and their targets are not recursively cleared. Concurrent ancestor/path replacement remains a documented limitation.

Từ chối root là symlink hoặc không phải directory, rồi xóa từng child trực tiếp. Entry symlink/junction bị xóa như link và target không bị xóa đệ quy. Race thay path/ancestor đồng thời vẫn là giới hạn được ghi nhận.

Implementation/error owner: `clear_tree.rs` and `error.rs`; tests: `test/`.
