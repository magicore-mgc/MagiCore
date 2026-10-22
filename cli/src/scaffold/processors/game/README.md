# CLI Game scaffold processor

This is the active built-in fallback used by `create-game` when no registry template layer is available. `GameProcessor` keeps the stable `supports`/`files` API and dispatches to one framework writer. The CLI create path first validates its framework and checks the registry layer; this module does not invoke external package-manager commands.

Đây là built-in fallback đang được `create-game` dùng khi không có registry template layer. `GameProcessor` giữ API `supports`/`files` và dispatch tới writer theo framework. Luồng CLI kiểm tra framework rồi thử registry layer trước; module này không gọi lệnh package manager bên ngoài.

Framework output owners: [Bevy](bevy/README.md), [Godot](godot/README.md), [Unity](unity/README.md), and [Unreal](unreal/README.md). Existing direct renderer checks are in [`delivery_descriptors.rs`](../../processor/test/delivery_descriptors.rs); the outer create flow is in [`game.rs`](../../../commands/core/create/game.rs).

Các owner output framework: [Bevy](bevy/README.md), [Godot](godot/README.md), [Unity](unity/README.md), [Unreal](unreal/README.md). Test renderer hiện có ở [`delivery_descriptors.rs`](../../processor/test/delivery_descriptors.rs); flow create bên ngoài ở [`game.rs`](../../../commands/core/create/game.rs).

Known follow-up: `files` currently treats an unknown framework as Bevy. Keep that behavior unchanged in the structural pass; add a typed fail-closed error and negative test in the later behavior-fix phase.
