# `adapters/` — Ecosystem Adapters (9 Cores)

Each adapter implements the `mgc-adapter-base` traits for a specific technology ecosystem.

## Overview

| Adapter | Crate | Ecosystems Supported |
|---|---|---|
| `web/` | `mgc-web-adapter` | Node.js, NPM, TypeScript, React, Vue, Next.js, FastAPI, Django, Spring Boot, Gin, Laravel, Symfony… |
| `ai/` | `mgc-ai-adapter` | Python AI/ML, LLM serving, Hugging Face, MCP framework scaffolding |
| `cloud/` | `mgc-cloud-adapter` | Terraform, Pulumi, AWS CDK, Cloudflare Workers, Serverless |
| `cicd/` | `mgc-cicd-adapter` | GitHub Actions, GitLab CI, ArgoCD, Docker Compose, Kubernetes |
| `game/` | `mgc-game-adapter` | Godot 4, Unity, Unreal Engine, Bevy (Rust) |
| `iot/` | `mgc-iot-adapter` | PlatformIO, Zephyr RTOS, ESP32, Arduino, STM32 |
| `app/` | `mgc-app-adapter` | Flutter, Swift Package Manager, Kotlin/Gradle, React Native |
| `lib/` | `mgc-lib-adapter` | Universal polyglot libraries (Rust crates + Python packages + npm packages) |
| `hardware/` | `mgc-hardware-adapter` | Hardware benchmarking, resource allocation, platform detection |

## Architecture

Each adapter follows the same owner-folder layout: a top-level `lib.rs` facade
plus one folder per responsibility, each with a same-named file and a `mod.rs`
that wires it (per RULE §2).

```
adapters/<name>/
├── Cargo.toml
└── src/
    ├── lib.rs          # Public facade: re-exports the adapter type
    ├── adapter/        # Adapter type + capability manifest (adapter/adapter.rs)
    ├── install/        # Package installation (install/<owner>/<owner>.rs)
    ├── manifest/       # Manifest parsing (package.json / mgc.toml / etc.)
    ├── audit/          # Security advisory checks
    └── ...             # One folder per responsibility; no flat install.rs
```

## `web/` — Reference Implementation

The web adapter is the most mature and serves as the reference implementation for all other adapters. It handles:

- Full npm-compatible package resolution + lockfile management
- CAS reflink/hardlink installation
- Lifecycle scripts (trust-gated, opt-in)
- 16+ framework scaffolds (React, Vue, Next.js, Vite, Express, NestJS, FastAPI, Django, Spring Boot, Gin, Axum, Fiber, Echo, Laravel, Symfony, Quarkus)
- Monorepo workspace support with `workspaces[]` in `package.json`

## Adding a New Adapter

1. Copy `adapters/lib/` as a starting point.
2. Implement all traits from `core/crates/mgc-adapter-base/`.
3. Register the adapter in `cli/src/dispatch/core/`.
4. Add integration tests under `adapters/<name>/tests/`.
