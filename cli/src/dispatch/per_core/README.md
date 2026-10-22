# Per-core CLI routing

Converts each shared or core-specific CLI command group into `DispatchCommand`.

## Vietnamese

Folder này chuyển từng nhóm lệnh CLI chung và từng core sang `DispatchCommand`. Mapping giữ riêng theo nhóm thao tác, bao phủ web, game, ai, clo, cicd, iot, app, lib và hardware. `add/`, `install/` và `list/` chỉ đổi schema CLI thành request; việc dựng `CoreCommand::*` do owner mapping dùng chung đảm nhiệm.
`common/` owns the 20 shared CLI command-to-dispatch mappings and their nearest payload tests; `engine/` owns engine-command mappings.

`common/` sở hữu 20 ánh xạ lệnh CLI dùng chung sang dispatch và test payload gần owner; `engine/` sở hữu ánh xạ lệnh engine.
`remove/` maps the explicit remove CLI variants to the shared nine-core constructor. It keeps the existing `Option<CoreCommand>` adapter contract.

`remove/` ánh xạ variant CLI remove tường minh vào constructor dùng chung cho chín core và giữ nguyên hợp đồng adapter `Option<CoreCommand>`.
`update/` maps explicit update CLI variants into the shared nine-core owner while preserving the existing `Option<CoreCommand>` route.

`update/` ánh xạ variant CLI update tường minh vào owner dùng chung cho chín core và giữ nguyên route `Option<CoreCommand>`.
`create/` owns the explicit nine-core scaffold-command adapter. It preserves the distinct Web, container and basic payload shapes and returns `None` for unrelated commands.

`create/` sở hữu adapter scaffold tường minh cho chín core. Owner giữ payload Web, container và basic theo đúng kiểu riêng, trả `None` cho lệnh không thuộc create.
