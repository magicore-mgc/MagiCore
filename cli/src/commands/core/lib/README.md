# Lib core owner

This module owns Lib-specific adapter construction, project-identity validation around context loading, and optional registry selection through `MAGICORE_LIB_REGISTRY_URL` / `MAGICORE_LIB_REGISTRY_TOKEN`. Add/install/update/remove/list command handlers call this owner. `core::shared::lib_adapter` remains a compatibility re-export while older internal callers migrate.

`error.rs` classifies the adapter-binding boundary and retains the original error as its source. `test/` contains the nearest unit-test sources. Test execution is intentionally deferred until the whole structural pass is ready.

# Owner của Lib core

Module này sở hữu việc dựng adapter riêng cho Lib, xác thực project identity quanh lúc tải context, và chọn registry tùy chọn qua `MAGICORE_LIB_REGISTRY_URL` / `MAGICORE_LIB_REGISTRY_TOKEN`. Các handler add/install/update/remove/list gọi owner này. `core::shared::lib_adapter` còn được giữ làm facade tương thích trong lúc chuyển callers cũ.

`error.rs` phân loại boundary dựng adapter và giữ lỗi gốc làm source. `test/` chứa test gần helper nhất. Chỉ chạy test sau khi hoàn tất đợt tái cấu trúc toàn bộ.
