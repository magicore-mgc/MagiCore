# Cloud type

Defines Cloud framework identity and precedence-ordered filesystem/manifest detection used by the adapter factory.

Định nghĩa định danh framework Cloud và phát hiện filesystem/manifest theo thứ tự ưu tiên mà adapter factory sử dụng.

Detection retains regular-file checks for framework markers and current precedence. `manifest_is_cloud` remains crate-private.

Phát hiện tiếp tục chỉ nhận regular file cho marker và giữ thứ tự ưu tiên hiện tại. `manifest_is_cloud` vẫn chỉ dùng trong crate.
