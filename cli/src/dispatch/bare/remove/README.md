# Bare remove adapter

This owner resolves the detected core and normalizes the bare `remove` CLI fields before calling `dispatch/remove_mapping/`. Its error type reports only an unrelated verb routed into this mapper. Ecosystem validation and unknown-core reporting stay with their existing owners.

Owner này phân giải core đã nhận diện và chuẩn hóa field CLI `remove` bare trước khi gọi `dispatch/remove_mapping/`. Error type chỉ báo verb khác bị route nhầm vào mapper; validation ecosystem và lỗi core lạ vẫn thuộc owner hiện có.

The nearest test module covers all nine names, package/compatibility forwarding, Web's `no_install` inversion, missing/unknown cores and the typed wrong-verb error.

Test gần owner bao phủ đủ chín tên, chuyển tiếp package/compatibility, phép đảo `no_install` của Web, core bị thiếu/core lạ và lỗi typed khi sai verb.
