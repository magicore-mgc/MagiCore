# Dedupe types

Owns the public Clap request and serializable report/entry records used by `mgc dedupe`. `mod.rs` only wires and re-exports them through `commands::dedupe::*`. Report field names and types are part of the existing output contract. `disk_saved_bytes` is a compatibility name for observed logical payload lengths under removed paths, not physical disk savings.

Sở hữu request Clap và record report/entry serialize công khai của `mgc dedupe`. `mod.rs` chỉ nối module và export lại qua `commands::dedupe::*`. Tên/kiểu field report thuộc contract output hiện tại. `disk_saved_bytes` là tên tương thích cho tổng payload logic quan sát dưới path đã xóa, không phải số dung lượng đĩa vật lý được thu hồi.
