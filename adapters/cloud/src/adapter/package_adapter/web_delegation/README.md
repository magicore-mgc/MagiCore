# Cloud embedded-Web delegation owner

Owns the existing `prepare_add`, age-gate and package preference forwarding to the optional embedded `WebAdapter`. Projects without that adapter keep the existing typed unsupported result or no-op behavior.

Sở hữu việc chuyển tiếp `prepare_add`, age-gate và package preference hiện có sang `WebAdapter` nhúng tùy chọn. Project không có adapter này vẫn giữ kết quả unsupported có type hoặc hành vi no-op như cũ.

The owner does not invoke external package-manager commands.
