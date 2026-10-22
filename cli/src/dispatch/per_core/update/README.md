# Per-core update adapter

This owner converts explicit `update-<core>` variants into the shared update request and constructor. It preserves the existing `Option<CoreCommand>` coordinator contract and all three input fields.

Owner này đổi variant `update-<core>` tường minh sang request và constructor update dùng chung. Nó giữ hợp đồng coordinator `Option<CoreCommand>` cùng cả ba field đầu vào.

Nearest tests exercise all nine explicit variants with both install values and present/absent compatibility values.

Test gần owner chạy qua đủ chín variant tường minh với cả hai lựa chọn install và compat có/không.
