#!/usr/bin/env bash
# Enforce fail-closed CI/release invariants. — Ép các invariant CI/release fail-closed.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ALL_CORE="$ROOT/.github/workflows/delegated-compatibility-matrix.yml"
RELEASE="$ROOT/.github/workflows/release.yml"
SECURITY="$ROOT/.github/workflows/security.yml"
VERIFY_DISTRIBUTION="$ROOT/.github/workflows/verify-release-distribution.yml"
LOCAL_RUNNER="$ROOT/scripts/run_all_core_tests.sh"

fail() {
  echo "FAIL: $1" >&2
  exit 1
}

# Stale validation runs must not consume runners after newer work arrives.
# Mỗi workflow có group riêng để không hủy nhầm workflow kiểm chứng khác.
for workflow in ci delegated-compatibility-matrix lifecycle-matrix nightly-audit-matrix release-binary-e2e security stress-matrix resolver-archive-security; do
  grep -Fq "group: ${workflow}-" "$ROOT/.github/workflows/${workflow}.yml" \
    || fail "${workflow} must isolate its concurrency group"
  grep -Fq 'cancel-in-progress: true' "$ROOT/.github/workflows/${workflow}.yml" \
    || fail "${workflow} must cancel superseded validation runs"
done

# Keep benchmark PMs isolated from runner-owned shims and use current Swift setup.
# Cô lập PM so sánh khỏi shim hệ thống; dùng cùng action Swift với lifecycle.
stress_workflow="$ROOT/.github/workflows/stress-matrix.yml"
grep -Fq 'npm install --global --prefix "$pm_prefix"' "$stress_workflow" \
  || fail "stress PM setup must use an isolated prefix"
grep -Fq 'swift-actions/setup-swift@364295d9c23900ce04d4e5cc708387921b4e50f9' "$stress_workflow" \
  || fail "stress Swift setup must use the current lifecycle installer"

for required_scaffold_test in \
  'cargo test --release --locked -p mgc --lib commands::template::tests' \
  'cargo test --release --locked -p mgc --lib scaffold::cache::tests' \
  'cargo test --release --locked -p mgc --lib scaffold::processor::tests' \
  'cargo test --release --locked -p mgc --test scaffold_integration_test --test all_core_parity_test'; do
  grep -Fq "$required_scaffold_test" "$ROOT/.github/workflows/lifecycle-matrix.yml" \
    || fail "the three-OS lifecycle matrix must execute ${required_scaffold_test}"
done

# Container assets belong to the MGC CLI flag path, never framework templates.
# Asset container do flag CLI MGC sinh, không nằm trong template framework.
[[ ! -e "$ROOT/scripts/gen-backend-templates.py" ]] \
  || fail "framework template generator must not create backend/container scaffold files"
if grep -Fq 'gen-backend-templates.py' "$ROOT/.github/workflows/ci.yml"; then
  fail "CI must not overlay generated backend files into framework templates"
fi
container_template_files="$(find "$ROOT/templates" -type f \( \
  -name Dockerfile -o -name Containerfile -o -name .dockerignore -o -name .containerignore \
  -o -name docker-compose.yml -o -name docker-compose.yaml -o -name compose.yml -o -name compose.yaml \
  -o -name mgc-container-entrypoint \) -print)"
[[ -z "$container_template_files" ]] \
  || fail "Docker assets must be emitted by MGC flags, not framework template files"
container_template_dirs="$(find "$ROOT/templates" -type d \( -name k8s -o -name kubernetes \) -print)"
[[ -z "$container_template_dirs" ]] \
  || fail "Kubernetes assets must be emitted by MGC flags, not framework template directories"

PYTHONDONTWRITEBYTECODE=1 python3 "$ROOT/scripts/test_release_lifecycle_gate.py" \
  || fail "release publishers must require the same-commit lifecycle workflow"

PYTHONDONTWRITEBYTECODE=1 python3 "$ROOT/tests/test_scaffold_acceptance_runner.py" \
  || fail "scaffold acceptance runner must reject failed commands and missing output"

python3 "$ROOT/scripts/test_release_script.py" || fail "release script argument handling regression tests failed"

grep -q 'run: bash scripts/test_ci_workflow_contract.sh' "$ROOT/.github/workflows/ci.yml" \
  || fail "CI must execute this contract before its release and action-pin assertions are treated as gates"
seeded_ci_registry="$(sed -n '/Seed registry + cargo test/,/^  build:/p' "$ROOT/.github/workflows/ci.yml")"
grep -Fq 'curl -sf -o /dev/null http://127.0.0.1:4315/v2/' <<<"$seeded_ci_registry" \
  || fail "seeded CI registry readiness must probe its registered OCI /v2/ ping route"
grep -Fq 'if [[ "$registry_ready" != 1 ]]; then' <<<"$seeded_ci_registry" \
  || fail "registry seeding must stop when the readiness budget expires"
grep -Fq 'if (fail > 0) process.exit(1);' <<<"$seeded_ci_registry" \
  || fail "historical fixture normalization must stop on unresolved links"
grep -Fq 'cargo test -p mgc-exec --test test_run -- --test-threads=1 2>&1' "$ROOT/.github/workflows/ci.yml" \
  || fail "process-table executor tests must serialize synthetic process fixtures"
if grep -Eq 'curl -sf -o /dev/null http://127\.0\.0\.1:4315/[[:space:]]' <<<"$seeded_ci_registry"; then
  fail "seeded CI registry readiness must not probe the unregistered root route"
fi
grep -Fq 'python3 "$ROOT/scripts/audit_workflow_action_pins.py"' "$ROOT/scripts/test_ci_workflow_contract.sh" \
  || fail "the CI-invoked workflow contract must audit every workflow action reference"
python3 "$ROOT/scripts/test_workflow_action_pins.py" || fail "workflow action-pin audit regression tests failed"
python3 "$ROOT/scripts/audit_workflow_action_pins.py" "$ROOT/.github/workflows" \
  || fail "a workflow action reference is not immutably pinned"
python3 "$ROOT/scripts/test_workflow_shell_inputs.py" \
  || fail "workflow shell-input audit regression tests failed"
python3 "$ROOT/scripts/audit_workflow_shell_inputs.py" "$ROOT/.github/workflows" \
  || fail "a workflow dispatch input is interpolated into a shell run block"
grep -Fq 'RELEASE_VERSION_INPUT: ${{ inputs.version }}' "$RELEASE" \
  || fail "release workflow must transfer the dispatch version through step env"
grep -Fq 'VERSION="$RELEASE_VERSION_INPUT"' "$RELEASE" \
  || fail "release workflow must use its safely transferred dispatch version"
grep -Fq 'if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then' "$RELEASE" \
  || fail "release workflow must validate the version before exporting it to later jobs"
grep -Fq 'RELEASE_TAG: ${{ inputs.tag }}' "$VERIFY_DISTRIBUTION" \
  || fail "distribution verifier must transfer the dispatch tag through step env"
grep -Fq 'TAG="$RELEASE_TAG"' "$VERIFY_DISTRIBUTION" \
  || fail "distribution verifier must use its safely transferred dispatch tag"

grep -q '^  pull_request:' "$ALL_CORE" || fail "delegated-compatibility matrix must run on pull requests"
grep -q '^  pull_request:' "$SECURITY" || fail "security workflow must run on pull requests touching audited inputs"
[[ "$(grep -Fc '"core/Cargo.lock"' "$SECURITY")" -ge 2 ]] || fail "security push and pull-request filters must cover the standalone core lockfile"
grep -q '"core/\*\*"' "$ALL_CORE" || fail "delegated-compatibility matrix path filter misses the core directory"
grep -q '"fix/\*\*"' "$ALL_CORE" || fail "delegated-compatibility matrix must run on RC fix branches"
grep -q '"fix/\*\*"' "$ROOT/.github/workflows/ci.yml" || fail "CI must run on RC fix branches"
grep -q '".github/workflows/security.yml"' "$SECURITY" || fail "security workflow changes must retrigger security checks"
grep -q '82a92a6e8fbeee089604da2575dc567ae9ddeaff' "$ROOT/.github/workflows/ci.yml" && fail "CI contains the invalid rust-cache SHA"
grep -q '6323deb102c322ba6fcbdcafc7e3dddab59af2b6' "$ROOT/.github/workflows/ci.yml" || fail "CI rust-cache pin must resolve to v2.9.2 (latest, node24)"
grep -q '7b1c307e0dcbda6122208f10795a713336a9b35a' "$ROOT/.github/workflows/ci.yml" && fail "CI contains the broken Rust toolchain pin"
grep -q 'e2a55d2ffb04f378e9626c28d38b36d230d1e12f' "$ROOT/.github/workflows/ci.yml" || fail "CI Rust toolchain action must use the immutable reviewed ref"
grep -q 'toolchain: "1.99.0"' "$ROOT/.github/workflows/ci.yml" || fail "CI Rust stable lane must use the current pinned stable version"
grep -q 'e2a55d2ffb04f378e9626c28d38b36d230d1e12f' "$ALL_CORE" || fail "delegated-compatibility Rust toolchain action must use the immutable reviewed ref"
grep -q 'toolchain: "1.99.0"' "$ALL_CORE" || fail "delegated-compatibility Rust lane must use the current pinned stable version"
grep -q 'python3 scripts/test_audit_dependency_delegation.py' "$ROOT/.github/workflows/ci.yml" || fail "CI must run dependency ownership audit negative controls"
grep -q 'python3 scripts/test_audit_capability_matrix.py' "$ROOT/.github/workflows/ci.yml" || fail "CI must test audit-matrix source provenance"
grep -q 'working_tree_clean.*is not True' "$RELEASE" || fail "release audit gate must reject dirty-tree matrices"
grep -q 'working_tree_clean.*is not True' "$ROOT/.github/workflows/nightly-audit-matrix.yml" || fail "nightly audit gate must reject dirty-tree matrices"
grep -q 'scripts/test_dep_gate_consistency.py' "$ROOT/.github/workflows/lifecycle-matrix.yml" || fail "lifecycle CI must test binary-to-matrix operation ownership"
grep -Fq 'run: bash cli/tests/scripts/runtime_full_e2e.sh' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must execute the real-binary dev and project audit-log E2E"
grep -Fq 'cargo build --release --bin mgc --locked' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "product lifecycle E2E must build the optimized mgc binary from the checked-out source"
grep -Fq 'cargo build --release --bin mgc --locked' "$ROOT/.github/workflows/ci.yml" \
  || fail "runtime lifecycle E2E must build the optimized mgc product binary"
grep -Fq 'MGC_BIN: ./target/release/mgc' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "real-binary E2E must execute the optimized release binary"
grep -Fq 'cargo test --locked -p mgc --test global_tool_e2e -- --test-threads=1 --nocapture' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must exercise persistent CLI tools on every runner OS"
grep -Fq 'cargo test --locked -p mgc --test global_tool_e2e platform::different_projects_share_one_user_tool_root_without_removing_each_other -- --ignored --exact --test-threads=1 --nocapture' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must execute the Node-backed shared global-tool-root E2E on every runner OS"
grep -Fq 'cargo test -p mgc --test global_tool_e2e --locked -- --include-ignored --test-threads=1 --nocapture' "$ROOT/.github/workflows/ci.yml" \
  || fail "Unix CLI lifecycle CI must run the ignored persistent tool tests with Node installed"
grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "persistent CLI tool E2E must invoke the release product binary"
grep -A4 -F 'Global CLI tool lifecycle E2E (Unix)' "$ROOT/.github/workflows/ci.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "global-tool lifecycle E2E must invoke the release product binary"
grep -A4 -F 'Run native test-runner security E2E on release binary' "$ROOT/.github/workflows/ci.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "test-runner security E2E must invoke the release product binary"
grep -A4 -F 'Run optimizer E2E on release binary' "$ROOT/.github/workflows/ci.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "optimizer E2E must invoke the release product binary"
grep -Fq 'cargo test -p mgc --test optimizer_lifecycle_e2e --locked -- --include-ignored --nocapture' "$ROOT/.github/workflows/ci.yml" \
  || fail "CI must run the Node-backed optimizer E2E after setup-node"
grep -Fq 'cargo test --locked -p mgc --test optimizer_lifecycle_e2e test_web_node_mgc_test_with_optimizer -- --ignored --exact --nocapture' "$ROOT/.github/workflows/stress-matrix.yml" \
  || fail "stress CI must run the exact Node-backed optimizer E2E"
grep -Fq 'cargo test -p mgc --test native_ownership_matrix_e2e framework_npm_version_probe_uses_mgc_shim_with_canonical_windows_cwd --locked -- --ignored --exact --test-threads=1 --nocapture' "$ROOT/.github/workflows/ci.yml" \
  || fail "Windows CI must run the exact Node-backed authenticated npm shim probe"
grep -Fq 'cargo test --locked -p mgc --test native_ownership_matrix_e2e framework_npm_version_probe_uses_mgc_shim_with_canonical_windows_cwd -- --ignored --exact --test-threads=1 --nocapture' "$ROOT/.github/workflows/stress-matrix.yml" \
  || fail "cross-OS stress CI must run the exact Node-backed authenticated npm shim probe"
grep -A5 -F 'Run Web, App scaffold, and Lib lifecycle tests on release binary' "$ROOT/.github/workflows/ci.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "Web/App scaffold/Lib lifecycle E2E must invoke the release product binary"
grep -A4 -F 'Run AI lifecycle test with MGC-installed pytest on release binary' "$ROOT/.github/workflows/ci.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "AI lifecycle E2E must invoke the release product binary"
grep -Fq 'cargo test --locked -p mgc --test audit_cli_e2e audit_ai_cli_scans_checkpoint_weights_after_models_directory -- --exact --nocapture' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must verify Windows-safe AI artifact audit through the product binary"
grep -Fq 'cargo test --locked -p mgc --test core_identity_cli_e2e -- --test-threads=1 --nocapture' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must verify renamed core-marker rejection through the product binary"
grep -A3 -F 'Run live IoT install and product build E2E' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "IoT lifecycle E2E must execute the release product binary"
grep -Fq 'cargo test --release --locked -p mgc --test native_ownership_matrix_e2e rust_lib_reuses_verified_shared_artifacts_across_project_roots -- --exact --test-threads=1 --nocapture' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must prove cross-project Rust store reuse on the release binary"
grep -A4 -F 'Verify Rust packages are reused across projects from the release binary' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  | grep -Fq 'MGC_TEST_CLI_BINARY: ${{ github.workspace }}/target/release/mgc' \
  || fail "cross-project package-store E2E must execute the release product binary"
grep -Fq 'Loaded Rust Lib optimizer config' "$ROOT/cli/tests/iot_native_install_e2e.rs" \
  || fail "IoT product E2E must prove the build consumes generated optimizer settings"
grep -Fq '.magicore/exec.log' "$ROOT/cli/tests/iot_native_install_e2e.rs" \
  || fail "IoT product E2E must prove the project-local execution audit is written"
grep -Fq 'store/cargo/registry' "$ROOT/cli/tests/iot_native_install_e2e.rs" \
  || fail "IoT product E2E must prove where shared Rust archives and sources are stored"
grep -Fq 'cargo test --locked -p mgc --test dlx_e2e -- --include-ignored --test-threads=1 --nocapture' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "lifecycle CI must explicitly execute the Node-backed dlx script-policy E2E on every runner OS"
grep -Fq 'MGC_LIFECYCLE_BIN: ./target/release/mgc' "$ROOT/.github/workflows/lifecycle-matrix.yml" \
  || fail "capability lifecycle matrix must probe the optimized release binary"
grep -Fq 'process-churn fixture did not spawn at least 100 short-lived children' "$ROOT/cli/tests/scripts/runtime_full_e2e.sh" \
  || fail "real-binary lifecycle E2E must retain its short-lived process-churn regression"
grep -Fq 'optimizer-generated Cargo job/profile settings affected real mgc build and test' "$ROOT/cli/tests/scripts/runtime_full_e2e.sh" \
  || fail "real-binary lifecycle E2E must prove optimizer settings affect actual Cargo build and test"
process_guard_job="$(sed -n '/^  check:/,/^  minimum-rust-version:/p' "$ROOT/.github/workflows/ci.yml")"
grep -Fq 'actions/setup-python@' <<<"$process_guard_job" \
  || fail "the process-guard test job must install its required Python runtime instead of skipping tests"
workspace_test_job="$(sed -n '/^  test:/,/^  build:/p' "$ROOT/.github/workflows/ci.yml")"
grep -Fq 'actions/setup-python@' <<<"$workspace_test_job" \
  || fail "the full workspace test job must install Python for Unix process-guard tests"
grep -q 'CI cannot reach github.com' "$ROOT/scripts/verify_github_action_pin.sh" || fail "CI must fail closed when upstream action pins cannot be verified"
grep -q 'CI cannot verify upstream action pins without git' "$ROOT/scripts/verify_github_action_pin.sh" || fail "CI must fail closed when git is unavailable for action-pin verification"
grep -q 'UNVERIFIED: cannot reach github.com' "$ROOT/scripts/verify_github_action_pin.sh" || fail "offline action-pin checks must be reported as unverified"
python3 "$ROOT/scripts/test_github_action_pin.py" || fail "action-pin verifier behavior tests failed"

cross_rev='64b5bb4d3d34de062552b9a2093affe77b4ad16a'
cross_install="cargo install cross --git https://github.com/cross-rs/cross --rev ${cross_rev} --locked"
grep -Fq "$cross_install" "$RELEASE" \
  || fail "release cross compiler install must use the reviewed immutable revision and Cargo.lock"
grep -Fq "$cross_install" "$ROOT/scripts/build_all_platforms.sh" \
  || fail "local cross build instructions must use the same immutable revision as release CI"

manifest_signing="$(sed -n '/^      # Release provenance (P0)/,/^      # Note: Attestations/p' "$RELEASE")"
grep -q 'MGC_RELEASE_REQUIRE_SIGNED: "1"' <<<"$manifest_signing" \
  || fail "release manifest signing must require a signature"
grep -Fq 'MGC_RELEASE_TRUST_ROOTS: ${{ vars.MGC_RELEASE_TRUST_ROOTS }}' <<<"$manifest_signing" \
  || fail "release workflow must verify the signed manifest with its configured public trust root"
grep -q 'scripts/verify-release-manifest-signature.py' <<<"$manifest_signing" \
  || fail "release workflow must cross-check its signature and every archive binding before creating a draft"
public_manifest_verify="$(sed -n '/^      - name: Verify signed release manifest using public trust roots/,/^      # Note: Attestations/p' "$RELEASE")"
grep -Fq 'MGC_RELEASE_TRUST_ROOTS: ${{ vars.MGC_RELEASE_TRUST_ROOTS }}' <<<"$public_manifest_verify" \
  || fail "the post-signing release verifier must receive the public trust-root variable"
if grep -q 'MGC_RELEASE_SIGNING_KEY' <<<"$public_manifest_verify"; then
  fail "the post-signing release verifier must not receive the private signing seed"
fi
if grep -Eiq 'without the secret.*(publishes|publish).*unsigned|publishes an unsigned manifest' <<<"$manifest_signing"; then
  fail "release workflow must not claim that unsigned manifests can be published when signing is mandatory"
fi

signed_distribution="$(sed -n '/^  verify-signed-manifest:/,$p' "$ROOT/.github/workflows/verify-release-distribution.yml")"
grep -q 'python3 scripts/verify-release-manifest-signature.py' <<<"$signed_distribution" \
  || fail "distribution verification must run the tested native signature-and-archive verifier"
grep -q 'MGC_RELEASE_TRUST_ROOTS:.*vars.MGC_RELEASE_TRUST_ROOTS' <<<"$signed_distribution" \
  || fail "distribution verification must use the configured public release trust root"
if grep -q 'MGC_RELEASE_SIGNING_KEY' <<<"$signed_distribution"; then
  fail "distribution verification must never receive the private release signing seed"
fi
grep -q 'contents: read' <<<"$signed_distribution" \
  || fail "post-publication distribution verification must use read-only repository access"
if grep -Eiq 'pip[[:space:]]+install.*cryptography|signature NOT verified|gh release download.*\|\|[[:space:]]*true' <<<"$signed_distribution"; then
  fail "signed distribution verification must not install an external verifier or waive missing evidence"
fi
grep -q 'magicore-\*\.tar.gz' <<<"$signed_distribution" \
  || fail "signed distribution verification must download every tar.gz archive for digest binding"
grep -q 'magicore-\*\.zip' <<<"$signed_distribution" \
  || fail "signed distribution verification must download every zip archive for digest binding"
grep -q 'every signed archive must be downloaded and verified' "$ROOT/scripts/verify-release-manifest-signature.py" \
  || fail "signed distribution verifier must reject omitted manifest-bound archives"
grep -q 'missing required release variants' "$ROOT/scripts/verify-release-manifest-signature.py" \
  || fail "signed distribution verifier must enforce required cross-platform variants"

homebrew_asset_download="$(sed -n '/^      - name: Download the formula shipped on the release/,/^      - name:/p' "$ROOT/.github/workflows/verify-release-distribution.yml")"
grep -Fq -- '--pattern "magicore.rb"' <<<"$homebrew_asset_download" \
  || fail "Homebrew release verifier must download the flat release asset name"
grep -Fq -- '--pattern "magicore-web.rb"' <<<"$homebrew_asset_download" \
  || fail "Homebrew release verifier must include the web-only formula asset"
grep -Fq 'cp downloaded/magicore.rb packaging/homebrew/magicore.rb' <<<"$homebrew_asset_download" \
  || fail "Homebrew verifier must restore the formula path after downloading the flat asset"
grep -Fq "grep -Eq 'PLACEHOLDER|PENDING_[A-Z0-9_]+|UPDATE_ME|0000000000000000' packaging/homebrew/*.rb" <<<"$homebrew_asset_download" \
  || fail "Homebrew verifier must reject placeholder hashes in both release formulas"
if ! printf '%s\n' 'PENDING_RC10_SHA256_DO_NOT_PUBLISH' | grep -Eq 'PLACEHOLDER|PENDING_[A-Z0-9_]+|UPDATE_ME|0000000000000000'; then
  fail "distribution placeholder detector must match the current RC checksum token"
fi

scoop_asset_download="$(sed -n '/^      - name: Download the manifest shipped on the release/,/^      - name:/p' "$ROOT/.github/workflows/verify-release-distribution.yml")"
grep -Fq -- '--pattern "magicore.json"' <<<"$scoop_asset_download" \
  || fail "Scoop release verifier must download the flat release asset name"
grep -Fq -- '--pattern "magicore-web.json"' <<<"$scoop_asset_download" \
  || fail "Scoop release verifier must include the web-only manifest asset"
grep -Fq 'cp downloaded/magicore.json packaging/scoop/magicore.json' <<<"$scoop_asset_download" \
  || fail "Scoop verifier must restore the bucket path after downloading the flat asset"
grep -Fq "grep -Eq 'PLACEHOLDER|PENDING_[A-Z0-9_]+|UPDATE_ME|0000000000000000' packaging/scoop/*.json" <<<"$scoop_asset_download" \
  || fail "Scoop verifier must reject placeholder hashes in both release manifests"

# Verify each published architecture, including Apple Silicon; select exactly
# one all-core archive so the web-only match cannot make ARCHIVE ambiguous.
# Kiểm từng kiến trúc phát hành, gồm Apple Silicon; chọn đúng archive all-core.
archive_verification="$(sed -n '/^  verify-archive-download:/,/^  verify-homebrew:/p' "$VERIFY_DISTRIBUTION")"
grep -q 'arch: arm64' <<<"$archive_verification" \
  || fail "public distribution verification must exercise the published macOS arm64 archive"
grep -A3 '^          - os: ubuntu-24.04-arm$' <<<"$archive_verification" | grep -q 'arch: arm64' \
  || fail "public distribution verification must exercise the published Linux arm64 archive on a native runner"
grep -q 'arch: x64' <<<"$archive_verification" \
  || fail "public distribution verification must retain x64 archive coverage"
macos_intel_target="$(grep -A2 '^          - os: macos-15-intel$' <<<"$archive_verification")"
grep -q 'arch: x64' <<<"$macos_intel_target" \
  || fail "the Intel macOS runner must verify the x64 archive"
macos_arm_target="$(grep -A2 '^          - os: macos-latest$' <<<"$archive_verification")"
grep -q 'arch: arm64' <<<"$macos_arm_target" \
  || fail "the Apple Silicon macOS runner must verify the arm64 archive"
grep -q 'EXT="zip"' <<<"$archive_verification" \
  || fail "Windows verification must select the zip archive format"
grep -q 'EXT="tar.gz"' <<<"$archive_verification" \
  || fail "Unix verification must select the tar.gz archive format"
grep -q 'ARCHIVE="magicore-${VERSION}-${OS_NAME}-${ARCH}.${EXT}"' <<<"$archive_verification" \
  || fail "verification must select exactly one all-core archive for its OS and architecture"
if grep -Eq 'ARCHIVE=.*ls magicore-[*]' <<<"$archive_verification"; then
  fail "distribution verification must not select ambiguous all-core and web-only archive matches"
fi

# The Linux ARM64 target must build and smoke-test on the native ARM runner.
# Target Linux ARM64 phải được build và smoke test trên runner ARM bản địa.
release_build_matrix="$(sed -n '/^  build-release:/,/^  test-release-artifacts:/p' "$RELEASE")"
linux_arm_build="$(grep -A6 '^          - os: ubuntu-24.04-arm$' <<<"$release_build_matrix" | head -n 7)"
grep -q 'target: aarch64-unknown-linux-gnu' <<<"$linux_arm_build" \
  || fail "release build matrix must target native Linux ARM64"
grep -q 'arch: arm64' <<<"$linux_arm_build" \
  || fail "Linux ARM64 release archive must carry the arm64 contract identity"
grep -q 'use_cross: false' <<<"$linux_arm_build" \
  || fail "Linux ARM64 release target must build natively on its hosted runner"
release_smoke_matrix="$(sed -n '/^  test-release-artifacts:/,/^  pre-publish-verification:/p' "$RELEASE")"
[[ "$(grep -c '^          - os: ubuntu-24.04-arm$' <<<"$release_smoke_matrix")" -eq 2 ]] \
  || fail "all-core and web Linux ARM64 artifacts must both have smoke-test rows"
linux_arm_smoke_blocks="$(grep -A3 '^          - os: ubuntu-24.04-arm$' <<<"$release_smoke_matrix")"
[[ "$(grep -c 'arch: arm64' <<<"$linux_arm_smoke_blocks")" -eq 2 ]] \
  || fail "Linux ARM64 smoke-test rows must use the matching architecture identity"

# Verify GitHub Actions SHA pins (real commit refs)
checkout_sha='3d3c42e5aac5ba805825da76410c181273ba90b1' # v7.0.1 real
setup_node_sha='949feb2413d6458794dcd2491c4babbbce0c15c1' # v7.1.0 real
setup_python_sha='5fda3b95a4ea91299a34e894583c3862153e4b97' # v7.0.0 real
setup_go_sha='b7ad1dad31e06c5925ef5d2fc7ad053ef454303e' # v7.0.0 real
setup_java_sha='b843ebf9b913146b38da2eaa7aeddf76074ae1ad' # v6.1.0 real
setup_dotnet_sha='a98b56852c35b8e3190ac28c8c2271da59106c68' # v6.0.0 real

grep -q "actions/checkout@${checkout_sha}" "$ALL_CORE" || fail "delegated-compatibility checkout SHA must be v7.0.1 real commit"
grep -q "actions/setup-node@${setup_node_sha}" "$ALL_CORE" || fail "delegated-compatibility setup-node SHA must be v7.1.0 real commit"
grep -q "actions/setup-python@${setup_python_sha}" "$ALL_CORE" || fail "delegated-compatibility setup-python SHA must be v7.0.0 real commit"
grep -q "actions/setup-go@${setup_go_sha}" "$ALL_CORE" || fail "delegated-compatibility setup-go SHA must be v7.0.0 real commit"

source "$ROOT/scripts/verify_github_action_pin.sh"
if command -v git >/dev/null 2>&1; then
    verify_pin actions/checkout v7.0.1 "$checkout_sha"
    verify_pin actions/setup-node v7.1.0 "$setup_node_sha"
    verify_pin actions/setup-python v7.0.0 "$setup_python_sha"
    verify_pin actions/setup-go v7.0.0 "$setup_go_sha"
    verify_pin actions/setup-java v6.1.0 "$setup_java_sha"
    verify_pin actions/setup-dotnet v6.0.0 "$setup_dotnet_sha"
    verify_pin Swatinem/rust-cache v2.9.2 '6323deb102c322ba6fcbdcafc7e3dddab59af2b6'
    verify_pin actions/upload-artifact v7.0.2 'cf430e030ddbb5b0abf93d22962f4752f3646cd9'
    verify_pin actions/download-artifact v8.0.2 '9000827ccba6bdab643e8b6fd33ac0654aef8333'
    verify_pin astral-sh/setup-uv v10.3.0 '1c37ad07a6a961277cf70c0d37d6f313000f5884'
    verify_pin softprops/action-gh-release v3.0.3 'efb35369e0ad2afab669f228072c1b0d510eae64'
    verify_pin actions/attest-build-provenance v4.2.2 '4d101475d8b20a2381f78447822ac1eab6504dd8'
    verify_pin subosito/flutter-action v2.23.0 '1a449444c387b1966244ae4d4f8c696479add0b2'
    verify_pin oven-sh/setup-bun v2.2.0 '0c5077e51419868618aeaa5fe8019c62421857d6'
    verify_pin denoland/setup-deno v2.0.5 '22d081ff2d3a40755e97629de92e3bcbfa7cf2ed'
else
    if [[ "${CI:-}" == "true" || "${CI:-}" == "1" || "${GITHUB_ACTIONS:-}" == "true" ]]; then
        fail "CI cannot verify upstream action pins without git"
    fi
    echo "UNVERIFIED: git is unavailable; upstream action SHA checks cannot pass" >&2
    exit 2
fi

setup_go_count="$(grep -c "actions/setup-go@${setup_go_sha}" "$ALL_CORE")"
[[ "$setup_go_count" -eq 4 ]] || fail "delegated-compatibility matrix must provision pinned Go for all four core jobs"
go_version_count="$(grep -c 'go-version: "1.27.2"' "$ALL_CORE")"
[[ "$go_version_count" -eq 4 ]] || fail "delegated-compatibility matrix must pin Go 1.27.2 for all four core jobs"
grep -q "actions/setup-go@${setup_go_sha}" "$RELEASE" || fail "release builds must provision pinned Go for esbuild-rs"
grep -q 'go-version: "1.27.2"' "$RELEASE" || fail "release builds must pin Go 1.27.2"
go_version_values=$(grep -rhoE 'go-version: "[^"]+"' "$ROOT/.github/workflows" | sort -u)
[[ "$go_version_values" == 'go-version: "1.27.2"' ]] || fail "all workflow Go toolchains must use the canonical Go 1.27.2 pin; found: ${go_version_values//$'\n'/, }"
go_archive_values=$(grep -oE 'go[0-9]+\.[0-9]+\.[0-9]+\.linux-amd64\.tar\.gz' "$RELEASE" | sort -u)
[[ "$go_archive_values" == 'go1.27.2.linux-amd64.tar.gz' ]] || fail "release scanner archive must match canonical Go 1.27.2; found: ${go_archive_values//$'\n'/, }"
grep -q 'GO_SHA256="ecbadb99091a3f46e31f5f934b068b1864eafa7995211b39eaddf76996045fe5"' "$RELEASE" \
  || fail "release scanner must pin the official Go 1.27.2 Linux AMD64 archive checksum"
go_sha_check_line=$(grep -n -m1 'sha256sum --check --strict -' "$RELEASE" | cut -d: -f1 || true)
go_extract_line=$(grep -n -m1 'sudo tar -C /usr/local -xzf /tmp/go.tgz' "$RELEASE" | cut -d: -f1 || true)
[[ -n "$go_sha_check_line" && -n "$go_extract_line" && "$go_sha_check_line" -lt "$go_extract_line" ]] \
  || fail "release scanner archive must pass SHA-256 verification before extraction"

if grep -Eq 'uses: [^ ]+@(v[0-9]+|main|master|stable|latest)([[:space:]]|$)' "$ALL_CORE"; then
  fail "delegated-compatibility workflow contains floating action references"
fi

grep -q 'App: SKIP' "$LOCAL_RUNNER" && fail "local all-core runner must not convert App failures into skips"
grep -q 'ALL CORES LIFECYCLE VERIFIED' "$LOCAL_RUNNER" || fail "local runner summary missing"
grep -q 'add-ai pytest@9.1.1' "$LOCAL_RUNNER" || fail "local AI lifecycle must install pytest through MGC"
if grep -Eq 'python[^[:cntrl:]]*-m pip|pip[0-9]* install.*(pytest|build|setuptools)' "$LOCAL_RUNNER"; then
  fail "local AI lifecycle must not install project packages outside MGC"
fi
grep -q 'publish=true' "$RELEASE" && fail "release instructions reference the removed publish input"

release_dry_run=$(sed -n '/^  dry-run-summary:/,$p' "$RELEASE")
grep -q 'Ready to publish: YES' <<<"$release_dry_run" && fail "release dry-run must not claim publish readiness without the tag-triggered publish gates"
grep -q 'Publish readiness: NOT ASSESSED (dry-run does not authorize publishing)' <<<"$release_dry_run" \
  || fail "release dry-run must state that it does not authorize publishing"

grep -q 'node-version: "26.12.0"' "$ALL_CORE" || fail "Node.js lifecycle pin is stale"
grep -q 'python-version: "3.15.0"' "$ALL_CORE" || fail "Python lifecycle pin is stale"
grep -q 'flutter-version: "3.47.7"' "$ALL_CORE" || fail "Flutter lifecycle pin is stale"
grep -Eq 'cargo test --locked -p mgc --test full_lifecycle_e2e test_ai_full_lifecycle -- .*--ignored --exact' "$ROOT/.github/workflows/ci.yml" \
  || fail "CI does not execute the ignored AI release-binary lifecycle test using MGC-installed pytest"
if grep -Eq 'pip install.*(pytest|build|setuptools)|pytest --version' "$ROOT/.github/workflows/ci.yml" "$ROOT/.github/workflows/release-binary-e2e.yml"; then
  fail "Python project test/build packages must be installed and verified through MGC"
fi
grep -Eq 'cargo test --locked -p mgc --test full_lifecycle_e2e test_web_full_lifecycle -- .*--ignored --exact' "$ROOT/.github/workflows/ci.yml" \
  || fail "CI provisions Node.js but does not execute the ignored Web release-binary lifecycle test"
if grep -vE '^[[:space:]]*#' "$ALL_CORE" | grep -Eq 'setup-uv|(^|[[:space:]])uv([[:space:]]|$)'; then
  fail "native AI lifecycle matrix must not provision or invoke uv"
fi

app_matrix="$(sed -n '/^  app-lifecycle:/,/^  lib-lifecycle:/p' "$ALL_CORE")"
grep -q 'windows-latest' <<<"$app_matrix" || fail "App lifecycle must cover Windows"

web_section="$(sed -n '/^  web-lifecycle:/,/^  ai-lifecycle:/p' "$ALL_CORE")"
ai_section="$(sed -n '/^  ai-lifecycle:/,/^  app-lifecycle:/p' "$ALL_CORE")"
app_section="$(sed -n '/^  app-lifecycle:/,/^  lib-lifecycle:/p' "$ALL_CORE")"
lib_section="$(sed -n '/^  lib-lifecycle:/,/^  lifecycle-summary:/p' "$ALL_CORE")"
rust_setup="$(grep -A1 -- '- name: Setup Rust' <<<"$lib_section")"
grep -q 'if:' <<<"$rust_setup" && fail "Rust setup cannot be conditional because every lib row builds mgc"
grep -q '"\$MGC_PARENT_BIN" build' <<<"$lib_section" || fail "Lib lifecycle must build through the OS-correct mgc binary"

grep -q '"\$MGC_PARENT_BIN" add-ai six@1.17.0' <<<"$ai_section" || fail "AI lifecycle must add a real package through the native mgc path"
grep -q '"\$MGC_PARENT_BIN" install' <<<"$ai_section" || fail "AI lifecycle must install the native lock through mgc"
grep -q 'python -m py_compile' <<<"$ai_section" || fail "AI lifecycle must syntax-check the scaffold with the provisioned Python runtime"

for section in "$web_section" "$ai_section" "$app_section" "$lib_section"; do
  grep -q 'MGC_BIN=./target/release/mgc.exe' <<<"$section" \
    || fail "every core lifecycle job must resolve mgc.exe on Windows"
done

if grep -Eq '\|\|[[:space:]]*(true|echo)|exit[[:space:]]+0[[:space:]]*#.*skip' "$ALL_CORE"; then
  fail "delegated-compatibility matrix contains a pass-through bypass"
fi

# Anti-overclaim: the workflow verifies Web/AI/App/Lib only. Its name and
# summary must not claim all 9 cores until Game/IoT/Hardware/Cloud/CI-CD
# lifecycle jobs actually exist in the matrix.
# Chống overclaim: workflow chỉ verify Web/AI/App/Lib — tên và summary
# không được claim 9 core trước khi job lifecycle của các core kia tồn tại.
if grep -Eq '^name:[[:space:]]+All-Core' "$ALL_CORE"; then
  fail "workflow name overclaims: only Web/AI/App/Lib have lifecycle jobs — keep the Delegated Compatibility name or add all 9 cores"
fi
if grep -q 'ALL CORES' "$ALL_CORE"; then
  fail "workflow summary overclaims 'ALL CORES' while covering only four cores"
fi

checkout_line="$(grep -n 'name: Checkout repository' "$RELEASE" | head -1 | cut -d: -f1)"
contract_line="$(grep -n 'name: Set artifact names' "$RELEASE" | head -1 | cut -d: -f1)"
[[ "$checkout_line" -lt "$contract_line" ]] || fail "release contract runs before checkout"

ref_count="$(grep -c 'GITHUB_REF_NAME' "$RELEASE" || true)"
[[ "$ref_count" -eq 1 ]] || fail "resolved release version must be reused downstream"

grep -q '10 SBOMs' "$RELEASE" || fail "release summary must require all ten SBOMs (5 platform pairs x all/web)"
grep -q 'Windows doesn.t generate SBOM' "$RELEASE" && fail "Windows SBOM must not be silently excluded"

grep -q 'cargo-audit --version 0.22.2' "$SECURITY" || fail "cargo-audit pin is stale"
grep -q 'OSV_VERSION="2.6.0"' "$SECURITY" || fail "OSV-Scanner pin is stale"

stress_workflow="$ROOT/.github/workflows/stress-matrix.yml"
for package_manager_version in 'npm@12.2.0' 'pnpm@12.11.2' '@yarnpkg/cli-dist@4.18.1'; do
  grep -Fq "$package_manager_version" "$stress_workflow" \
    || fail "stress matrix must provision and verify latest package manager: $package_manager_version"
done
for package_manager_check in \
  'test "$(npm --version)" = "12.2.0"' \
  'test "$(pnpm --version)" = "12.11.2"' \
  'test "$(yarn --version)" = "4.18.1"' \
  'test "$(bun --version)" = "1.4.3"' \
  'deno --version | head -1 | grep -F "deno 2.9.7"'; do
  grep -Fq "$package_manager_check" "$stress_workflow" \
    || fail "stress matrix must verify installed package manager version: $package_manager_check"
done

# T0.5 installer freshness: the embedded SHA-256 in dev/cicd.rs must be
# non-empty AND must match scripts/install-from-gh.sh at the pinned
# commit — a stale checksum fails the pipeline it generates.
# (Portable sed: BSD grep on macOS runners has no -P.)
# (Tươi checksum installer: SHA-256 nhúng phải khác rỗng VÀ khớp file ở
# commit ghim — checksum cũ làm fail pipeline nó sinh ra.)
installer_sha="$(sed -n 's/.*MGC_INSTALLER_SHA: &str = "\([0-9a-f][0-9a-f]*\)".*/\1/p' "$ROOT/cli/src/commands/core/dev/cicd.rs" | head -n 1)"
[[ -n "$installer_sha" ]] || fail "installer commit pin is missing"
embedded_sha256="$(grep -A1 'MGC_INSTALLER_SHA256: &str' "$ROOT/cli/src/commands/core/dev/cicd.rs" | sed -n 's/.*"\([0-9a-f][0-9a-f]*\)".*/\1/p' | head -n 1)"
[[ -n "$embedded_sha256" ]] || fail "installer checksum is not pinned (MGC_INSTALLER_SHA256 empty)"
expected_sha256=$(git -C "$ROOT" show "${installer_sha}:scripts/install-from-gh.sh" | shasum -a 256 | awk '{print $1}')
[[ "$embedded_sha256" == "$expected_sha256" ]] || fail "installer checksum is stale for the pinned commit"

echo "PASS: CI workflow contract"
