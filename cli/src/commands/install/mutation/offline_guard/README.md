# Offline install preflight / Tiền kiểm install offline

This owner enforces the existing offline lock requirement, project trust policy, lock signature result handling, cache invalidation on tampering, and rejection of package-add arguments. It runs before package-count/ownership validation and before the mutation gateway, preserving the original fail-closed order. It reuses `PackageCache` and shared trust/error facilities; no local error type is needed. The nearest install suite has no dedicated offline-mutation case; coverage is deferred to the later test phase.

Owner giữ nguyên yêu cầu lock khi offline, trust policy/chữ ký, vô hiệu hóa cache khi lock bị sửa và từ chối thêm package offline. Nó chạy trước kiểm tra số lượng/ownership và trước mutation gateway. Lỗi dùng facility chung; test riêng cho offline mutation được ghi nhận cho phase sau.
