# Web npm dependency provider / npm dependency provider web

This module preserves the public `provider::*` API used by the Web adapter, CLI and shared resolver. Its `mod.rs` only declares the behavior owners and re-exports `NpmDependencyProvider`, `AgePolicy`, `eligible_versions`, `check_pinned_version`, and the crate-visible timestamp parser. Every owner has a same-name Rust implementation and its own nearest test folder.

| Owner | Responsibility |
|---|---|
| `provider` | Provider state, constructors, per-instance policy arming and cache invalidation. |
| `release_age`, `npm_timestamp`, `eligible_versions`, `pinned_version` | Age-policy type, timestamp parsing, candidate filtering and explicit pin validation. |
| `metadata`, `metadata_prefetch` | Metadata cache, alias source mapping and bounded concurrent fetch. |
| `dependency_graph` | npm alias parsing and dependency/peer edge conversion. |
| `platform_support`, `version_selection`, `enqueue_policy` | Host package compatibility, semver selection and resolver admission. |
| `prefetch_versions`, `prefetch_dependencies`, `dependency_provider` | Resolver prefetch operations and the shared resolver trait boundary. |

No owner creates a replacement error contract: registry failures keep using `DependencyError`, and the age-policy functions keep their existing `String` diagnostics. Runtime tests are deferred under the current source-first and SSD constraints.

Module này giữ nguyên API public `provider::*` mà Web adapter, CLI và resolver dùng chung đang gọi. `mod.rs` chỉ khai báo owner và export `NpmDependencyProvider`, `AgePolicy`, `eligible_versions`, `check_pinned_version` cùng parser timestamp crate-visible. Mỗi owner có implementation Rust cùng tên và thư mục test gần nhất riêng.

Các error contract không bị thay thế: lỗi registry tiếp tục dùng `DependencyError`; hàm age policy giữ nguyên diagnostic `String`. Test runtime được hoãn theo yêu cầu source-first và giới hạn SSD hiện tại.
