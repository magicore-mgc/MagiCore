# Version domain / Miền Version

Root `version/mod.rs` is a public namespace aggregator for the existing `Version` type. Each behavior is implemented in a same-name owner; local helpers stay private. The root has no single behavior, so it is the documented namespace-only exception to the implementation naming rule.

Root `version/mod.rs` là aggregator namespace công khai cho type `Version` hiện có. Mỗi hành vi nằm trong owner cùng tên; helper nội bộ giữ private. Root không có một hành vi đơn lẻ nên là ngoại lệ namespace được ghi rõ với quy tắc file implementation cùng tên.
