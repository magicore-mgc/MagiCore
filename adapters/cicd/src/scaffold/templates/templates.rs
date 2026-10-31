//! Pipeline templates and the single placeholder renderer they all share.
//! Template pipeline và renderer placeholder duy nhất dùng chung.
//!
//! These are the bytes `mgc ci-generate` writes. They live in the adapter so
//! the offline `create-cicd` generator and the in-project regenerator emit the
//! SAME pipeline instead of two YAML documents that drift apart. Every
//! placeholder substitution goes through `render_ci_template_with_repo`; a
//! template never reaches disk half-rendered.
//! Đây chính là bytes `mgc ci-generate` ghi. Chúng nằm ở adapter để generator
//! offline `create-cicd` và lệnh sinh lại trong project phát ra CÙNG một
//! pipeline, thay vì hai tài liệu YAML lệch nhau. Mọi thay placeholder đi qua
//! `render_ci_template_with_repo`; template không bao giờ chạm đĩa khi render dở.

/// Pin generated CI to the exact version of the MagiCore binary generating it.
/// (Workflow mới ghim đúng version binary MagiCore đang sinh ra nó.)
pub const MGC_RELEASE_TAG: &str = concat!("v", env!("CARGO_PKG_VERSION"));

/// Install source used by generated CI templates: the latest GitHub Release
/// installer downloaded from an IMMUTABLE commit SHA — never from the mutable
/// `main` branch (P0-E, 2026-09-16 supply-chain gate). Update the SHA to the
/// last commit that touched `scripts/install/install-from-gh.sh` whenever
/// `MGC_RELEASE_TAG` moves.
/// (P0-E: installer trong template CI được tải từ commit SHA BẤT BIẾN — không
/// bao giờ từ branch `main` mutable.)
pub const MGC_INSTALLER_SHA: &str = "51acbbb942d48407af82f5282fed0ffd5038516d";

/// Embedded SHA-256 of `scripts/install/install-from-gh.sh` at
/// `MGC_INSTALLER_SHA`.
///
/// Contract (P0-E/T0.5):
/// - empty → generation is refused (SEC-10): an unpinned installer lets the
///   pipeline install whatever the tag points at today;
/// - a `.sha256` file published next to the installer is verified whenever
///   present (mismatch fails the pipeline);
/// - when set, a checksum mismatch FAILS the pipeline (fail-closed).
///
/// Equal to `sha256(git show <MGC_INSTALLER_SHA>:scripts/install/install-from-gh.sh)`.
/// (P0-E/T0.5: SHA-256 nhúng của installer tại `MGC_INSTALLER_SHA`; rỗng →
/// từ chối sinh (SEC-10); lệch checksum → FAIL.)
pub const MGC_INSTALLER_SHA256: &str =
    "903f620f989e0314007ed8258523c42a39d05c3dfadba8b49bb91603b45bb9db";

/// GitHub Actions workflow (`.github/workflows/ci.yml`).
/// Workflow GitHub Actions (`.github/workflows/ci.yml`).
pub const WORKFLOW_TEMPLATE: &str = r#"name: {name}

on:
  push:
  pull_request:

jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
      - name: Install MagiCore (release binary, SHA-pinned)
        run: |
          set -eu
          pin_dir="$(mktemp -d)"
          trap 'rm -rf "$pin_dir"' EXIT
          installer="$pin_dir/install-from-gh.sh"
          # P0-E: fetch from an immutable commit SHA into a file — never
          # from mutable main, never piped straight into bash.
          curl -fsSL "https://raw.githubusercontent.com/magicore-mgc/MagiCore/{installer_sha}/scripts/install-from-gh.sh" -o "$installer"
          if curl -fsSL "https://raw.githubusercontent.com/magicore-mgc/MagiCore/{installer_sha}/scripts/install-from-gh.sh.sha256" -o "$pin_dir/install-from-gh.sh.sha256"; then
            (cd "$pin_dir" && { sha256sum -c install-from-gh.sh.sha256 || shasum -a 256 -c install-from-gh.sh.sha256; })
          else
            echo "WARNING: no .sha256 checksum file published next to the installer — companion verification skipped"
          fi
          if [ -n "{installer_sha256}" ]; then
            echo "{installer_sha256}  $installer" | { sha256sum -c - || shasum -a 256 -c -; }
          else
            echo "WARNING: installer integrity NOT pinned (MGC_INSTALLER_SHA256 empty) — trusting commit SHA {installer_sha} only"
          fi
          bash "$installer" --version {tag}
      - name: Check MagiCore version
        run: mgc --version
      - name: Install dependencies
        run: mgc install
      - name: Verify (strict audit in CI)
        env:
          MGC_AUDIT_STRICT: "1"
        run: mgc verify
"#;

/// GitLab CI pipeline (`.gitlab-ci.yml`).
/// Pipeline GitLab CI (`.gitlab-ci.yml`).
pub const GITLAB_TEMPLATE: &str = r#"stages:
  - ci

ci:
  stage: ci
  image: rust:latest
  before_script:
    - |
      set -eu
      pin_dir="$(mktemp -d)"
      trap 'rm -rf "$pin_dir"' EXIT
      installer="$pin_dir/install-from-gh.sh"
      # P0-E: fetch from an immutable commit SHA into a file — never from
      # mutable main, never piped straight into bash.
      curl -fsSL "https://raw.githubusercontent.com/magicore-mgc/MagiCore/{installer_sha}/scripts/install-from-gh.sh" -o "$installer"
      if curl -fsSL "https://raw.githubusercontent.com/magicore-mgc/MagiCore/{installer_sha}/scripts/install-from-gh.sh.sha256" -o "$pin_dir/install-from-gh.sh.sha256"; then
        (cd "$pin_dir" && { sha256sum -c install-from-gh.sh.sha256 || shasum -a 256 -c install-from-gh.sh.sha256; })
      else
        echo "WARNING: no .sha256 checksum file published next to the installer — companion verification skipped"
      fi
      if [ -n "{installer_sha256}" ]; then
        echo "{installer_sha256}  $installer" | { sha256sum -c - || shasum -a 256 -c -; }
      else
        echo "WARNING: installer integrity NOT pinned (MGC_INSTALLER_SHA256 empty) — trusting commit SHA {installer_sha} only"
      fi
      bash "$installer" --version {tag}
    - mgc --version
  script:
    - mgc install
    - MGC_AUDIT_STRICT=1 mgc verify
"#;

/// CircleCI pipeline (`.circleci/config.yml`).
/// Pipeline CircleCI (`.circleci/config.yml`).
pub const CIRCLE_TEMPLATE: &str = r#"version: 2.1
jobs:
  ci:
    docker:
      - image: cimg/base:stable
    steps:
      - checkout
      - run:
          name: Install MagiCore (release binary, SHA-pinned)
          command: |
            set -eu
            pin_dir="$(mktemp -d)"
            trap 'rm -rf "$pin_dir"' EXIT
            installer="$pin_dir/install-from-gh.sh"
            # P0-E: fetch from an immutable commit SHA into a file — never
            # from mutable main, never piped straight into bash.
            curl -fsSL "https://raw.githubusercontent.com/magicore-mgc/MagiCore/{installer_sha}/scripts/install-from-gh.sh" -o "$installer"
            if curl -fsSL "https://raw.githubusercontent.com/magicore-mgc/MagiCore/{installer_sha}/scripts/install-from-gh.sh.sha256" -o "$pin_dir/install-from-gh.sh.sha256"; then
              (cd "$pin_dir" && { sha256sum -c install-from-gh.sh.sha256 || shasum -a 256 -c install-from-gh.sh.sha256; })
            else
              echo "WARNING: no .sha256 checksum file published next to the installer — companion verification skipped"
            fi
            if [ -n "{installer_sha256}" ]; then
              echo "{installer_sha256}  $installer" | { sha256sum -c - || shasum -a 256 -c -; }
            else
              echo "WARNING: installer integrity NOT pinned (MGC_INSTALLER_SHA256 empty) — trusting commit SHA {installer_sha} only"
            fi
            bash "$installer" --version {tag}
      - run: mgc --version
      - run: mgc install
      - run:
          name: Verify (strict audit)
          command: MGC_AUDIT_STRICT=1 mgc verify
workflows:
  version: 2
  ci:
    jobs:
      - ci
"#;

/// Render a CI template: release tag + immutable installer pin (P0-E).
/// Render template CI: tag release + ghim installer bất biến (P0-E).
pub fn render_ci_template(template: &str, name: &str) -> String {
    render_ci_template_with_repo(template, name, "{repo_url}")
}

/// Render a template that also needs the project's repository URL.
/// Render template cần thêm URL repository của project.
pub fn render_ci_template_with_repo(template: &str, name: &str, repo_url: &str) -> String {
    // SEC-10: an unpinned installer digest is not a warning, it is a missing
    // release: a template must never be written with an empty pin.
    // (SEC-10: digest installer không pin là release thiếu, không phải cảnh
    // báo; template không bao giờ được ghi với pin rỗng.)
    debug_assert!(
        !MGC_INSTALLER_SHA256.is_empty(),
        "MGC_INSTALLER_SHA256 must be pinned in the release build"
    );
    template
        .replace("{repo_url}", repo_url)
        .replace("{name}", name)
        .replace("{tag}", MGC_RELEASE_TAG)
        .replace("{installer_sha}", MGC_INSTALLER_SHA)
        .replace("{installer_sha256}", MGC_INSTALLER_SHA256)
}

/// Refuse to generate when the release pin is missing (SEC-10).
/// Từ chối sinh khi thiếu pin release (SEC-10).
pub fn ensure_release_pin() -> mgc_types::MgResult<()> {
    if MGC_INSTALLER_SHA256.trim().is_empty() || MGC_INSTALLER_SHA.trim().is_empty() {
        return Err(mgc_types::MgError::Unsupported {
            core: "cicd",
            capability: "ci-generate",
            guidance: "refusing to generate CI because the installer pin is empty in this \
                       build (MGC_INSTALLER_SHA/MGC_INSTALLER_SHA256); rebuild from a \
                       released source tree"
                .to_string(),
        });
    }
    Ok(())
}
