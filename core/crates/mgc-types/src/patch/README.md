# Shared patch contracts

This module exports two related but distinct contracts: config-time `PatchSpec`/`PatchKind` and the persisted `LockPatch` record. The root keeps `mgc_types::patch::*` and the crate keeps its root re-exports.

Module này export hai contract có liên quan nhưng riêng biệt: `PatchSpec`/`PatchKind` cho cấu hình và record `LockPatch` được lưu. Root giữ `mgc_types::patch::*` và crate giữ các re-export hiện tại.

Separate owners keep config metadata apart from lockfile state. Neither owner fetches or applies patch files, and both remain infallible data models.

Owner riêng giữ metadata cấu hình tách khỏi trạng thái lockfile. Không owner nào tải hay áp dụng file patch; cả hai vẫn là model dữ liệu không có nhánh lỗi.

This type layer does not invoke an external patch binary; application belongs to a separate lifecycle owner.

Lớp type này không gọi binary patch bên ngoài; việc áp dụng patch thuộc một owner lifecycle riêng.
