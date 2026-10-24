# GitHub workflow owners / Owner GitHub workflow

## Why workflow files stay flat

GitHub Actions discovers workflow definitions from `.github/workflows/`. Keep executable workflow YAML files at this level. Split large workflows through `workflow_call` files instead of nesting YAML in subfolders, which GitHub does not discover as workflows. Put longer shell and PowerShell behavior in focused owners under `scripts/` and document that script folder there.

GitHub Actions nhận workflow từ `.github/workflows/`. Các file YAML workflow phải nằm trực tiếp ở đây. Tách workflow lớn bằng các file `workflow_call` thay vì đưa YAML vào folder con, vì GitHub không nhận chúng như workflow. Phần shell/PowerShell dài được chuyển vào owner cụ thể trong `scripts/` và mô tả tại README của script folder.

## Release topology / Luồng release

`release.yml` owns tag/manual triggers, version resolution, the stable 12 job IDs, dependency order, release gates, publisher conditions and caller permissions. Keep the three evidence/source gates in this entry so their exact `needs.*.result` checks remain visible at the publication boundary.

The job-level reusable owners are:

| Workflow | Responsibility |
|---|---|
| `release-build.yml` | Build the release matrix and upload named artifacts |
| `release-artifact-smoke.yml` | Download/extract the artifacts and smoke-test their Web install path |
| `release-prepublish-verification.yml` | Verify archive set, checksums, attestations and SBOMs |
| `release-audit-capability.yml` | Enforce current-commit audit capability evidence |
| `release-publish-container.yml` | Publish the CLI image to GHCR |
| `release-publish-github.yml` | Create the draft GitHub release and upload signed assets |
| `release-public-download-smoke.yml` | Verify public release downloads after publication |
| `release-dry-run-summary.yml` | Report a manual non-publishing run |

Each called workflow declares typed inputs/secrets and the minimum job permission it needs. Do not use `secrets: inherit`. Keep publisher `needs`, fail-closed conditions and the exact secret mapping in `release.yml`. See [`scripts/release/README.md`](../../scripts/release/README.md) for operation-level owners.

`release.yml` sở hữu trigger tag/manual, resolve version, 12 job ID ổn định, thứ tự phụ thuộc, release gate, điều kiện publisher và permission ở caller. Giữ ba evidence/source gate trong file này để `needs.*.result` được kiểm tra rõ ngay tại ranh giới publish.

Các reusable workflow theo job được liệt kê trong bảng tiếng Anh phía trên. Mỗi workflow khai báo input/secret có kiểu và permission tối thiểu. Không dùng `secrets: inherit`. Giữ `needs`, điều kiện fail-closed và ánh xạ secret chính xác ở `release.yml`. Owner theo thao tác nằm tại [`scripts/release/README.md`](../../scripts/release/README.md).

Trong `release-build.yml`, `GOARCH` và `CGO_ENABLED` chỉ được đặt ở bước native Unix. Không export `CC`/`CFLAGS` toàn cục: cgo của esbuild-rs cần Go target chính xác, còn cờ kiến trúc toàn cục có thể làm hỏng host proc-macro và ring. Go tự thêm `-arch` đúng cho Darwin khi cgo bật; chi tiết thao tác nằm trong `scripts/release/build/README.md`.

## CI Docker lifecycle / Lifecycle Docker trong CI

The `docker-cli-smoke` job remains in `ci.yml` to keep its check identity and ordered cross-core product coverage visible. Long container operations are owned by `scripts/ci/docker/` folders for common, Web, AI, Cloud, Lib, App and Kubernetes lanes. The workflow still owns Flutter/kind setup and each step boundary; see the script owner README for environment and deferred test requirements.

Job `docker-cli-smoke` vẫn nằm trong `ci.yml` để giữ check identity và phạm vi kiểm thử sản phẩm đa core theo thứ tự. Các thao tác container dài do `scripts/ci/docker/` sở hữu theo lane common, Web, AI, Cloud, Lib, App và Kubernetes. Workflow vẫn quản lý setup Flutter/kind và ranh giới từng step; README script owner mô tả môi trường và yêu cầu test đang hoãn.

## Stress matrix runtime and core lanes / Runtime và lane core của stress matrix

`stress-matrix.yml` remains the owner of its Ubuntu/macOS/Windows matrix, pinned setup actions, conditions, step environment, timeouts, E2E ordering and evidence upload. Fifteen operation bodies live under `scripts/ci/stress/` for OS-specific toolchains, version checks, CLI preparation and Go/App/Web lifecycle; the byte-identical Swift key importer is shared from `scripts/ci/toolchains/swift/` with lifecycle-matrix. Both script roots are included in stress push/PR filters. Runtime and workflow tests remain NOT_RUN until the global production-source pass closes.

`stress-matrix.yml` vẫn sở hữu ma trận Ubuntu/macOS/Windows, setup action ghim phiên bản, điều kiện, environment từng step, timeout, thứ tự E2E và upload evidence. Mười lăm body nằm dưới `scripts/ci/stress/` cho setup toolchain theo OS, kiểm tra version, chuẩn bị CLI và lifecycle Go/App/Web; importer Swift key giống hệt được stress/lifecycle dùng chung từ `scripts/ci/toolchains/swift/`. Cả hai path script đều có trong filter push/PR của stress. Test runtime/workflow vẫn NOT_RUN đến khi hoàn tất source production toàn repo.

## Lifecycle matrix Swift/App setup / Setup Swift/App của lifecycle matrix

`lifecycle-matrix.yml` retains the OS matrix, pinned actions, guards and order. Workflow-specific Windows Swift Python preservation/install and Flutter Linux prerequisites live in `scripts/ci/lifecycle/`; the exact Linux key-import operation is shared from `scripts/ci/toolchains/swift/`. The release-binary Swift installer and its key-import retry policy remain separate. Inline Python source assertions still require migration during the later test phase.

`lifecycle-matrix.yml` giữ ma trận OS, action ghim, guard và thứ tự. Swift Python preservation/cài trên Windows cùng prerequisite Flutter Linux riêng workflow nằm trong `scripts/ci/lifecycle/`; thao tác nhập key Linux giống hệt được dùng chung từ `scripts/ci/toolchains/swift/`. Installer Swift release-binary và chính sách retry key import của nó vẫn riêng. Assertion source Python đang đọc block inline phải được chuyển ở pha test sau.

The workflow's global v1.2 evidence gate now delegates to `scripts/lifecycle_matrix/validation/release_gate/`; per-lane console output delegates to `scripts/lifecycle_matrix/ci/`. Workflow step names/order, matrix, artifact upload and failure boundary remain at the GitHub-discovered YAML owner. The 143 lifecycle verdict test methods now live in `scripts/lifecycle_matrix/validation/test/`; `scripts/test/test_native_pm_verdict.py` remains the stable unittest entrypoint for CI. No test or workflow run has verified this extraction yet.

Gate evidence toàn cục v1.2 của workflow được chuyển sang `scripts/lifecycle_matrix/validation/release_gate/`; log từng lane chuyển sang `scripts/lifecycle_matrix/ci/`. Tên/thứ tự step, matrix, artifact upload và ranh giới lỗi vẫn thuộc YAML được GitHub phát hiện. 143 test method lifecycle verdict hiện nằm trong `scripts/lifecycle_matrix/validation/test/`; `scripts/test/test_native_pm_verdict.py` giữ entrypoint unittest ổn định cho CI. Chưa chạy test/workflow để xác minh lần tách này.

## Distributed binary lifecycle / Lifecycle của binary phân phối

`release-binary-e2e.yml` owns its Ubuntu/macOS/Windows matrix, pinned toolchain actions, setup conditions and ordered lifecycle steps. Long operations live in `scripts/release/binary_e2e/`: archive creation/installation, Web project phases, Rust/AI/Swift/Flutter lanes and signed Swift toolchain setup. The Web coordinator exports one temporary project root to its create/audit, dev probe, run probe and cache/offline phases. Exact environment and evidence limits are documented in that script owner's README.

`release-binary-e2e.yml` sở hữu ma trận Ubuntu/macOS/Windows, action toolchain ghim phiên bản, điều kiện setup và thứ tự lifecycle. Các thao tác dài nằm trong `scripts/release/binary_e2e/`: đóng/cài archive, các pha dự án Web, lane Rust/AI/Swift/Flutter và setup toolchain Swift có chữ ký. Coordinator Web truyền cùng root dự án tạm qua các pha create/audit, dev probe, run probe và cache/offline. README của script owner ghi rõ môi trường và giới hạn bằng chứng.

## Validation ownership and limits / Owner xác minh và giới hạn

`stress-matrix.yml`, `lifecycle-matrix.yml` and `release-binary-e2e.yml` path filters include the extracted release workflows; the stress workflow also watches `scripts/ci/stress/**`, and the release-binary E2E watches `scripts/release/**`. When adding another workflow/script owner, update the relevant push and pull-request paths in the same change.

This structure has only had static source review in the current source-first phase. YAML parsing, actionlint, GitHub execution, reusable-workflow check names, same-run artifact visibility, script tests and release E2E remain `NOT_RUN`/`NOT_VERIFIED` until the repository-wide production-source pass closes. A static pass does not establish a release can publish.

`stress-matrix.yml`, `lifecycle-matrix.yml` và `release-binary-e2e.yml` đã thêm path filter cho reusable release workflow; release-binary E2E cũng theo dõi `scripts/release/**`. Khi thêm owner workflow/script, cùng thay đổi phải cập nhật path push và pull request tương ứng.

`stress-matrix.yml` cũng theo dõi `scripts/ci/stress/**`; release-binary E2E theo dõi `scripts/release/**`. Khi thêm owner workflow/script khác, phải cập nhật push và pull request path tương ứng trong cùng thay đổi.

Trong pha source-first hiện tại mới rà source tĩnh. YAML parse, actionlint, chạy GitHub, tên check của reusable workflow, artifact visibility trong cùng run, script test và release E2E vẫn `NOT_RUN`/`NOT_VERIFIED` cho đến khi tách xong production source toàn repository. Rà tĩnh không chứng minh luồng release publish chạy thành công.
