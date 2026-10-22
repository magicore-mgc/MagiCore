go_sdk_path_file="${RUNNER_TEMP}/mgc-e2e-go-sdk-path"
python3 scripts/ci/stress/lanes/go/resolve_go_sdk_path.py > "$go_sdk_path_file"
IFS= read -r go_sdk_path < "$go_sdk_path_file"
export MGC_E2E_GO_BINARY="$go_sdk_path"
cargo test --locked -p mgc-exec --test test_run \
  real_go_readonly_run_survives_cold_compiler_processes \
  -- --ignored --test-threads=1
cargo test --locked -p mgc-resolver --test test_protocols_go \
  materialized_go_module_imports_with_real_sdk_without_rewriting_hash \
  -- --ignored --test-threads=1
