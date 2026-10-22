# Stress CLI build and snapshot / Build và snapshot CLI cho stress

`build_release.sh` configures the macOS host C/C++ compiler needed by Go-backed crates and builds the release CLI. `snapshot_release.sh` copies the all-core binary into `RUNNER_TEMP` and exports that exact path for later E2Es.

`build_release.sh` cấu hình compiler C/C++ host trên macOS cần cho crate dùng Go rồi build CLI release. `snapshot_release.sh` copy binary all-core vào `RUNNER_TEMP` và export đúng đường dẫn cho các E2E sau.
