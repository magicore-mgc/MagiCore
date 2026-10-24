# Initializers — official framework scaffolding

This owner answers one question: **which program does framework X publish to create a new project, and how is it invoked?**

It holds no framework source. Every record names a package the framework team owns (`create-vite`, `create-next-app`, `sv`, `create-astro`, `@angular/cli`, …). MagiCore installs that package into an isolated workspace, runs it through the shared package runtime, and keeps only the metadata it owns.

Owner này trả lời một câu hỏi: **framework X phát hành chương trình nào để tạo project mới, và gọi nó thế nào?**

Nó không chứa source framework. Mỗi record nêu một package do framework team sở hữu. MagiCore cài package đó vào workspace cô lập, chạy qua package runtime dùng chung, và chỉ giữ metadata nó sở hữu.

## Why there is no template in this repository

A package manager ships an executor, not a starter. `pnpm create vite` runs Vite's own `create-vite`; `bun create next` runs Vercel's `create-next-app`. MagiCore follows the same rule: when a framework updates its starter, no MagiCore release is needed.

Package manager ship executor, không ship starter. `pnpm create vite` chạy `create-vite` của Vite; `bun create next` chạy `create-next-app` của Vercel. MagiCore theo đúng quy tắc đó: framework cập nhật starter thì không cần phát hành MagiCore.

## Layout

| Path | Owns |
|---|---|
| `types/` | `InitializerRecord`, `ArgSlot`, `Layout`, `VersionPolicy`, `Mode` |
| `catalog/` | The records themselves — pure data, no code generation |
| `policy/` | Refuses any record whose package or binary is unsafe |
| `initializers.rs` | `lookup`, `supported` |

The executor lives next door in `../initializer/`, which calls `package_runtime` and validates the generated tree (no symlinks, inside the sandbox).

Executor nằm cạnh ở `../initializer/`, gọi `package_runtime` và kiểm cây sinh ra (không symlink, nằm trong sandbox).

## Adding a framework

1. Confirm the framework publishes an npm initializer. If it does not, do not add a record — the scaffold will fail closed with an honest message.
2. Add one entry to `catalog/catalog.rs` with its package, binary, identity, argument slots and layout.
3. Pass an install-suppression flag (`--skip-install`, `--no-install`, `--noInstall`) when the initializer offers one, so `mgc install` remains the only writer of the dependency graph.
4. Run `cargo test -p mgc initializers`. The catalog tests enforce a single `Project` slot, unique ids and the install-suppression rule.

## What MagiCore writes (not the initializer)

`mgc.toml`, `mgc.lock`, `.mgc.core`, `.mgc-provenance.json`, `sbom.cyclonedx.json`, and `.gitignore`/`README.md` only when the framework left them absent.

## Not supported by this path

`create-astro` probes the host package manager with `npm config get`; MagiCore rejects that call by design, so the initializer runs with `npm_config_user_agent` set and skips its own detection. `fastify-cli` always runs `npm init -y` with no opt-out, so it has no record and fails closed. Frameworks without a published initializer (Django, Laravel, Spring, Quarkus, Rails) need a different mechanism and are not served here.
