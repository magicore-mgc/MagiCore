# Game core command support / Hỗ trợ lệnh Game core

This folder owns Game-core command behavior. Its current production path installs the Bevy optimizer through the existing native Game adapter and preserves the `game` feature gate. This module split changes code ownership only; it does not claim support for every game engine.

Folder này sở hữu hành vi lệnh Game core. Luồng production hiện tại cài Bevy optimizer qua Game adapter native và giữ feature gate `game`. Việc chia module chỉ đổi nơi sở hữu mã; không tuyên bố hỗ trợ mọi game engine.

## Owner map / Bản đồ owner

```text
game/
├── mod.rs                    # stable Game command exports
├── optimizer_template/       # lock, stage, mutate, publish, cleanup coordinator
├── project_validation/       # Game adapter identity and regular Cargo.toml preflight
├── manifest_snapshot/        # exact manifest snapshot and rollback
├── optimizer_destination/    # optimizer destination link/type checks
├── optimizer_dependency/     # Cargo dependency edit and persisted-value readback
├── error/                    # existing typed optimizer errors
└── staging/                  # existing safe staging and cleanup
```

The transaction order is: validate Bevy project and native manifest; acquire the shared mutation lock; snapshot `Cargo.toml`; validate the destination; stage/materialize the optimizer; edit and read back the dependency; revalidate and publish; remove staging. Failed manifest mutation or publication restores the original manifest bytes and cleans staging through the existing typed-error path.

Thứ tự transaction là: xác nhận project Bevy và manifest native; lấy shared mutation lock; snapshot `Cargo.toml`; kiểm tra đích; stage/materialize optimizer; sửa và đọc xác nhận dependency; kiểm tra lại rồi publish; xóa staging. Nếu sửa manifest hoặc publish lỗi, hệ thống khôi phục byte manifest ban đầu và dọn staging qua typed-error path hiện có.

## Verification status / Trạng thái xác minh

Source ownership and static review are tracked in [S492](../../../../../docs/specs/wholeprojectrefactor/tracking/batches/S492-gameOptimizerOwners.md). The moved tests and newly authored owner tests are **NOT_RUN** during the requested source-first phase; no runtime or product acceptance is implied.

Việc tách owner và review tĩnh được theo dõi tại [S492](../../../../../docs/specs/wholeprojectrefactor/tracking/batches/S492-gameOptimizerOwners.md). Test được chuyển và test owner mới đều **NOT_RUN** trong pha source-first; chưa có kết luận nghiệm thu runtime hay product.
