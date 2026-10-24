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

## Runtimes

A record says *how* the framework produces a project:

| Runtime | Meaning | Frameworks |
|---|---|---|
| `RegistryPackage` | MagiCore installs the package the framework team publishes, then runs its binary in a sandbox | 24 frontend, 3 backend |
| `HttpArchive` | MagiCore asks the framework's own generator service for a project archive and extracts it | `spring-boot` (`start.spring.io`), `quarkus` (`code.quarkus.io`) |

`HttpArchive` needs no local toolchain and no package manager at all: the
framework team's service renders the project. Archive entries are validated
before anything is written — absolute paths, `..` traversal and Windows drive
prefixes are rejected, so a compromised service cannot escape the scaffold
workspace. `unwrap_single_root` handles services that wrap the project in one
extra directory.

## Frameworks with a record (29)

**Frontend / fullstack (24):** `vanilla` · `react` · `react-vite` · `vue` · `vue-vite` · `solid` · `solidjs` · `qwik` · `svelte` · `sveltekit` · `nextjs` · `next` · `nuxt` · `angular` · `remix` · `astro` · `vite` · `preact` · `lit` · `vue-cli` · `docusaurus` · `vitepress` · `turbo` · `tauri`

**Backend (3):** `express` · `nestjs` · `nest`

**JVM via generator service (2):** `spring-boot` · `quarkus` — scaffold works end to end; `mgc install` fails closed because a Maven parent/BOM is not resolved yet, which is a resolver gap rather than a scaffold gap.

Each one was executed end to end on macOS arm64 with MagiCore's own dependency ecosystem (`mgc install`) and its own build (`mgc build`).

## Why a framework can be advertised but have no record

`framework_registry` lists every framework MagiCore knows how to describe. A record requires more than that: the published initializer must be able to run without a package manager MagiCore forbids. These are listed as supported names but have no record, and `mgc create-web` fails closed with the honest source error:

| Framework | Reason |
|---|---|
| `fastify` | `fastify-cli` runs `npm init -y` unconditionally, which MagiCore denies |
| `hono` | `create-hono` prompts interactively even with `--template` |
| `t3` | `create-t3-app` calls `npm -v` even with `--noInstall` |
| `trpc` | same initializer as `t3` |
| `gatsby` | `gatsby-cli new` exits 1 in a non-TTY shell |
| `nx` | `create-nx-workspace` installs with npm unconditionally |
| `electron` | `create-electron-app` runs `npm install --save-dev` |
| `expo` | `create-expo-app` spawns a package manager despite `--no-install` |
| `react-native` | `@react-native-community/cli init` fails in a non-TTY shell |
| `solid-start` | `create-solid --solidstart` produces no project |
| `koa` | `create-koa-app` has no install suppression |

Adding any of these becomes possible the moment its initializer gains a non-interactive, non-installing mode. The catalog test enforces the install-suppression rule so a record cannot claim support it does not have.

`create-astro` probes the host package manager with `npm config get`; MagiCore rejects that call by design, so the initializer runs with `npm_config_user_agent` set and skips its own detection.

Frameworks without a published initializer (Django, Laravel, Spring, Quarkus, Rails) need a different mechanism and are not served here.
