# Repository scripts / Script kho mã nguồn

Every script has exactly one owner folder. The root of `scripts/` holds no loose
files: if you are looking for a script, it is in the folder that owns its job.
Workflow YAML, `packaging/` manifests and docs reference the folder path, never
a bare `scripts/foo.sh`.

Mỗi script thuộc đúng một owner folder. Root `scripts/` không còn file lẻ: cần
tìm script thì xem folder sở hữu công việc đó. Workflow YAML, manifest
`packaging/` và docs trỏ theo đường dẫn folder, không trỏ `scripts/foo.sh` trần.

## Owners

| Folder | Owns / Sở hữu |
|---|---|
| `audit/` | Read-only repository checks: capability matrix, dependency delegation, mutation call inventory, workflow pins and shell inputs, module hygiene, no-package-manager rules, lifecycle capability matrix. |
| `test/` | Local and CI test entry points: core test sweep, CLI smoke, workflow contract tests, coverage manifest validation, SBOM validation. |
| `release/` | Release pipeline: manifest build/sign/verify, hash updates, provenance, publish, prepublish verification, artifact contract, and the tests that pin those programs. |
| `install/` | End-user installation: `install.sh`, `install.ps1`, `install-from-gh.sh`, Flutter SDK bootstrap. |
| `bench/` | Benchmark and cross-platform build drivers. |
| `maintenance/` | One-off repository upkeep: build cache cleaning, fixture generation, structure/test migration, split-source filling, GitNexus MCP, Ed25519 helper. |
| `ci/` | Shell/PowerShell operations invoked by GitHub Actions (docker, lifecycle, stress, toolchains). |
| `release/*` sub-owners | Nested programs that belong to the release pipeline (`artifact_smoke/`, `binary_e2e/`, `build/`, `prepublish/`). |
| `dependency_delegation_audit/` | Python package behind `audit/audit_dependency_delegation.py`. |
| `lifecycle_matrix/` | Python package behind `audit/lifecycle_capability_matrix.py`. |
| `docs/` | Internal notes for the scripts owners; not product documentation. |

## Rules

1. A new script goes into the folder that owns its behavior, not the root.
2. Scripts resolve the repository root from their own location. Owners are one
   level deeper than before, so `dirname "$0"/..` is no longer the repo root.
3. A script may call a sibling in the same folder with `"$SCRIPT_DIR/name"`.
   Crossing into another owner folder makes the dependency-delegation audit
   treat the call as dynamic, so prefer moving the shared program into the
   owner that uses it.
4. Moving a script means updating its callers: `.github/workflows/`,
   `packaging/`, adapter READMEs and `scripts/dependency_delegation_audit/rules/`
   review tables (`SHELL_DEPENDENCY_COMMAND_REVIEWS`,
   `SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS`, `SHELL_SOURCE_DYNAMIC_COMMAND_REVIEWS`).
   Review fingerprints bind file content, so editing a reviewed script requires
   re-review, not an allowlist edit alone.
5. `scripts/audit/audit_dependency_delegation.py` and
   `scripts/audit/audit_mutation_calls.py` are gates. Both must report
   `unreviewed=0` and `violation=0` after any script move.

## Vietnamese

1. Script mới vào folder sở hữu hành vi của nó, không đặt ở root.
2. Script tự xác định repo root từ vị trí của nó. Owner giờ sâu hơn một cấp nên
   `dirname "$0"/..` không còn là repo root.
3. Script được gọi sibling cùng folder bằng `"$SCRIPT_DIR/name"`. Gọi sang
   folder owner khác khiến audit dependency-delegation coi là dynamic, nên hãy
   chuyển chương trình dùng chung sang owner đang dùng nó.
4. Đổi vị trí script phải cập nhật caller: `.github/workflows/`, `packaging/`,
   README adapter và các bảng review trong
   `scripts/dependency_delegation_audit/rules/`
   (`SHELL_DEPENDENCY_COMMAND_REVIEWS`, `SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS`,
   `SHELL_SOURCE_DYNAMIC_COMMAND_REVIEWS`). Fingerprint review gắn với nội dung
   file, nên sửa script đã review cần review lại chứ không chỉ sửa allowlist.
5. `scripts/audit/audit_dependency_delegation.py` và
   `scripts/audit/audit_mutation_calls.py` là gate. Cả hai phải báo
   `unreviewed=0` và `violation=0` sau mọi lần di chuyển script.
