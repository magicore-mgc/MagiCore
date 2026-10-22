dart_sdk_path_file="${RUNNER_TEMP}/mgc-e2e-dart-sdk-path"
python3 scripts/ci/stress/lanes/app/resolve_dart_sdk_path.py > "$dart_sdk_path_file"
IFS= read -r dart_sdk_path < "$dart_sdk_path_file"
export MGC_E2E_DART_BINARY="$dart_sdk_path"
cargo test --locked --no-default-features --features app -p mgc \
  --test app_native_install_e2e flutter_offline_replay_imports_package_with_real_dart \
  -- --ignored --test-threads=1
