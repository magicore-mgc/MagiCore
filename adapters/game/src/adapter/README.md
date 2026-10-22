# Game adapter owners

## Purpose / Mục đích

This folder decomposes GameAdapter by stable responsibility while keeping the crate's public facade in `src/lib.rs`. The adapter recognizes Bevy, Godot, Unity and Unreal; it only claims ProjectDetector, ScaffoldProvider and AuditProvider. It does not execute a compatibility package manager.

Thư mục này tách GameAdapter theo từng trách nhiệm nhưng giữ facade công khai ở `src/lib.rs`. Adapter nhận diện Bevy, Godot, Unity và Unreal; chỉ claim ProjectDetector, ScaffoldProvider và AuditProvider. Adapter không chạy package manager tương thích bên ngoài.

## Owners / Các owner

- [`adapter.rs`](adapter.rs): engine state.
- [`capability_manifest`](capability_manifest/README.md): claimed capability list.
- [`identity`](identity/README.md): CoreIdent and public engine access.
- [`project_detection`](project_detection/README.md): game project detection.
- [`scaffold_provider`](scaffold_provider/README.md): scaffold capability probe.
- [`content_store_provider`](content_store_provider/README.md): explicit unsupported store/install paths.
- [`lockfile_provider`](lockfile_provider/README.md): Bevy-only native manifest writing.
- [`dependency_resolver`](dependency_resolver/README.md): unsupported add/remove/update routes.
- [`audit_provider`](audit_provider/README.md): shared audit delegation.
- [`package_adapter`](package_adapter/README.md): manifest identity/parse/list implementation.
- [`factory`](factory/README.md): `adapter_for` constructor.
- [`unclaimed_capabilities`](unclaimed_capabilities/README.md): fail-closed marker traits.

## Errors and tests / Lỗi và kiểm thử

Owners reuse shared `MgError`/`MgResult`; there is no owner-specific error type to split out. Tests of the exported adapter and its consumer contract remain in [`adapters/game/tests/integration.rs`](../../tests/integration.rs), because that file verifies the crate-root public API. Scaffold tests remain under `src/scaffold/test/`. Do not create empty test folders or duplicate the integration assertions.
