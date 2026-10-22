# Shared adapter contract

This module is the stable public data and trait boundary shared by every ecosystem adapter, CLI dispatch, capability traits and audit reporting. `mod.rs` re-exports the original `mgc_types::adapter::*` names; `package_adapter/package_adapter.rs` owns the central `PackageAdapter` trait; every DTO and the facade has a nearest-owner test module. The facade stays one trait because splitting it would change the public implementer contract.

Module này là ranh giới trait và dữ liệu public dùng chung bởi mọi ecosystem adapter, CLI dispatch, capability trait và audit report. `mod.rs` re-export nguyên các tên `mgc_types::adapter::*`; `adapter.rs` sở hữu trait trung tâm `PackageAdapter`.

Keep serialization tags/defaults, fail-closed trait defaults, audit clean-state rules and value-based manifest identity stable. Each type owner has its own README and same-name implementation file. Existing callers, the `capabilities/` namespace and crate-root re-exports remain outside this source-only slice.

Giữ ổn định tag/default serialization, trait default fail-closed, quy tắc trạng thái audit sạch và manifest identity so theo value. Mỗi type owner có README và file implementation trùng tên riêng. Caller hiện tại, `capabilities.rs` và re-export crate root nằm ngoài lát source-only này.
