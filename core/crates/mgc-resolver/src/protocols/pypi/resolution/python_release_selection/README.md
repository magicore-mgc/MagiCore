# python_release_selection / python_release_selection

Selects the newest release that supports the configured consumer Python and has an installable artifact. It fetches version-specific JSON only for candidates needed by the existing selection loop.

Chọn release mới nhất hỗ trợ consumer Python đã cấu hình và có artifact cài được. Chỉ tải JSON theo version cho candidate mà vòng chọn hiện tại cần.

The exact Python-version gate, Requires-Python diagnostics, artifact compatibility check, fallback order and metadata reuse are preserved.

Owner giữ nguyên cổng phiên bản Python, diagnostic Requires-Python, kiểm tra artifact, thứ tự fallback và việc tái sử dụng metadata.
