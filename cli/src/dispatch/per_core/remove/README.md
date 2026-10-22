# Per-core remove adapter

This owner converts explicit `remove-<core>` CLI variants to the shared remove request and constructor. It preserves `Option<CoreCommand>` for the coordinator and forwards Web's reinstall choice only to the Web variant.

Owner này đổi variant CLI `remove-<core>` tường minh sang request và constructor remove dùng chung. Nó giữ kiểu trả về `Option<CoreCommand>` cho coordinator và chỉ chuyển lựa chọn cài lại vào variant Web.

Its nearest tests exercise all nine explicit core variants and both Web reinstall values. The module does not perform package operations or policy checks.

Test gần owner chạy qua đủ chín variant core tường minh và hai giá trị cài lại của Web. Module không thực hiện package operation hoặc kiểm tra policy.
