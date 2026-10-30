# remedy

EN: Owns the next-step guidance a missing scaffold layer prints. MagiCore has no `mgc template` subcommand, so guidance names only remedies that exist: for `game`/`iot`/`clo` the adapter's offline generator first (they scaffold offline now), then `--from-local <path>`, the framework's official initializer, and the registry settings actually read by `select_registry` (`--registry`, `MGC_NPM_REGISTRY`, `.npmrc`). `names_nonexistent_subcommand` (test-only) guards the invariant.

VI: Sở hữu hướng dẫn bước tiếp theo khi thiếu scaffold layer. MagiCore không có subcommand `mgc template`, nên hướng dẫn chỉ nêu cách có thật: với `game`/`iot`/`clo` generator offline của adapter đứng trước (các core này scaffold offline), rồi `--from-local <path>`, initializer chính thức của framework, và cấu hình registry thật sự được `select_registry` đọc (`--registry`, `MGC_NPM_REGISTRY`, `.npmrc`). `names_nonexistent_subcommand` (chỉ test) giữ bất biến này.
