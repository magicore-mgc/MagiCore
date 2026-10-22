pub(in crate::commands::core::dev::cicd) const CIRCLE_TEMPLATE: &str = r#"version: 2.1
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
            curl -fsSL "https://raw.githubusercontent.com/mingd-153/MagiCore/{installer_sha}/scripts/install-from-gh.sh" -o "$installer"
            if curl -fsSL "https://raw.githubusercontent.com/mingd-153/MagiCore/{installer_sha}/scripts/install-from-gh.sh.sha256" -o "$pin_dir/install-from-gh.sh.sha256"; then
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
