# Lifecycle capability matrix

This package builds evidence for all 22 lifecycle lanes across lib, web, AI, app, game, IoT, cloud/orchestration, hardware and CI/CD. The root script remains the CLI and import compatibility path. `api.py` re-exports the stable consumer surface; lane data is grouped by core and reassembled in original order.

Fail-closed behavior is part of the contract: missing toolchains stay unverified, malformed evidence fails, unsupported operations remain explicit, and sandbox/process cleanup semantics are retained. Python and OS exceptions are caught only at the existing policy boundaries; no blanket exception type suppresses errors.

`validation/release_gate/` owns the global v1.2 lifecycle/native-PM gate; `ci/` owns presentation-only per-lane logs. The workflow invokes these owners after generating and uploading the matrix. Their source contracts and runtime checks remain deferred until the repository-wide test phase.

## Tiếng Việt

Package này tạo evidence cho đủ 22 lifecycle lane thuộc lib, web, AI, app, game, IoT, cloud/orchestration, hardware và CI/CD. Script root vẫn là đường CLI/import tương thích. `api.py` giữ mặt API ổn định; lane data chia theo core rồi ghép lại đúng thứ tự cũ.

Fail-closed là hợp đồng: thiếu toolchain ghi unverified, evidence lỗi thì thất bại, operation unsupported vẫn ghi rõ, cleanup sandbox/process được giữ nguyên. Chỉ bắt lỗi tại boundary đã có, không thêm catch-all để che lỗi.

`validation/release_gate/` sở hữu gate lifecycle/native-PM toàn cục v1.2; `ci/` chỉ sở hữu log trình bày từng lane. Workflow gọi hai owner này sau khi tạo và upload matrix. Source contract test và runtime sẽ được làm ở pha test toàn repository.
