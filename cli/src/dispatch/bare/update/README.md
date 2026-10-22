# Bare update adapter

This owner resolves the detected core and normalizes the bare `update` arguments before calling `dispatch/update_mapping/`. It preserves the existing missing/unknown-core behavior and owns the typed wrong-verb error.

Owner này phân giải core đã nhận diện và chuẩn hóa argument `update` bare trước khi gọi `dispatch/update_mapping/`. Nó giữ nguyên hành vi core thiếu/core lạ và sở hữu lỗi typed khi sai verb.

Nearest tests cover all nine core names, package/install/compatibility forwarding and local diagnostics.

Test gần owner bao phủ đủ chín tên core, chuyển tiếp package/install/compatibility và diagnostic cục bộ.
