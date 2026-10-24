# MagiCore Distribution

This directory contains package metadata and release checks. The authoritative release matrix and tag workflow are defined in [`.github/workflows/release.yml`](../.github/workflows/release.yml).

## Published variants

The release workflow publishes two binaries:

- `magicore`: all nine core families.
- `magicore-web`: web core only.

Each is built for five targets: Linux x64 and arm64, macOS x64 and arm64, and Windows x64. This produces ten archives, each with a SHA-256 sidecar and a CycloneDX SBOM. Windows ARM64 is not in the current release matrix.

The nine package manifests under `packaging/packages/` also let `mgc-dist` build core-specific binaries locally. Those seven additional core-specific variants are not published as GitHub release archives.

## Install a published binary

Use a version shown on the [GitHub Releases page](https://github.com/mingd-153/MagiCore/releases). Replace `<published-version>` below with the version without its leading `v`:

```bash
curl -fsSLo /tmp/mgc-install-from-gh.sh \
  https://raw.githubusercontent.com/mingd-153/MagiCore/main/scripts/install/install-from-gh.sh
VERSION="<published-version>" # replace with a version listed on GitHub Releases
bash /tmp/mgc-install-from-gh.sh \
  --package magicore \
  --version "$VERSION" \
  --dir "$HOME/.local/bin"
```

Use `--package magicore-web` for the web-only binary. The installer downloads the matching archive and `.sha256` sidecar, verifies the archive checksum, extracts `mgc`, and installs it into the requested directory. SHA-256 detects a mismatch against the downloaded sidecar; the installer does not verify the signed release manifest.

For Windows x64 in PowerShell, download and verify the ZIP before extracting it:

```powershell
$Version = "<published-version>" # replace with a version listed on GitHub Releases
$Archive = "magicore-$Version-windows-x64.zip"
$Url = "https://github.com/mingd-153/MagiCore/releases/download/v$Version/$Archive"
Invoke-WebRequest $Url -OutFile $Archive
Invoke-WebRequest "$Url.sha256" -OutFile "$Archive.sha256"
$Expected = (Get-Content -Raw "$Archive.sha256").Trim().Split()[0].ToLowerInvariant()
$Actual = (Get-FileHash -Path $Archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($Actual -ne $Expected) { throw "SHA-256 mismatch" }
$InstallDir = Join-Path $env:USERPROFILE ".magicore\bin"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -LiteralPath $Archive -DestinationPath $InstallDir -Force
& (Join-Path $InstallDir "mgc.exe") --version
```

Add the extraction directory to the user PATH if the shell should find `mgc` by name.

The source files in `packaging/homebrew/` and `packaging/scoop/` are release inputs. The release workflow finalizes their checksums and attaches copies to the GitHub Release, but it does not update external package-manager repositories. Live GitHub inspection on 2026-10-09 confirmed that the owner’s [`homebrew-tap`](https://github.com/mingd-153/homebrew-tap) still points to MagiCore `1.0.0-rc.2` and legacy archive names; no Scoop bucket repository appeared in the owner’s repository list. Do not advertise `brew install magicore` or `scoop install magicore` for the 1.1.0 RC until those channels are updated and verified.

## Release workflow

Pushing a version tag matching the workspace version starts `.github/workflows/release.yml`. The workflow builds both published variants for the five targets, creates archives/checksums/SBOMs, runs pre-publish checks, signs `manifest.json`, creates a **draft** GitHub Release, and smoke-tests the draft assets before a maintainer publishes it. After publication, run `.github/workflows/verify-release-distribution.yml` manually with that tag; it checks public downloads and package-manager installation using read-only repository access.

Before a tag-triggered release can pass, configure repository Actions secret `MGC_RELEASE_SIGNING_KEY` with a 64-character hexadecimal Ed25519 seed and repository Actions variable `MGC_RELEASE_TRUST_ROOTS` with its matching 64-character hexadecimal public key. The variable accepts comma-separated public keys for rotation. CI signs with the secret and verifies with the public variable; it never passes the private seed to the distribution verifier. Clients that run `mgc self-update` must pin the same public key with `--trust-root` or `MGC_RELEASE_TRUST_ROOTS`. Never commit the seed or put it in an issue or workflow log.

`workflow_dispatch` runs the workflow in dry-run mode and does not publish a release. The tag-triggered path intentionally starts as a draft; a maintainer reviews the completed checks and promotes it manually. The public-distribution workflow must run after promotion because Homebrew/Scoop fetch archives from the public release URL.

The current Homebrew/Scoop checksum placeholders in source manifests are intentional fail-closed values. Do not publish them directly. The release job replaces them using checksums computed from the completed release archives and verifies the generated manifests before attaching them.

## Local build scope

```bash
bash scripts/release/release.sh --profile release
```

This command runs `mgc-dist build-all` and creates nine core-specific binaries plus build receipts for the host target under `dist/`. It does not create the ten cross-platform release archives; those are built by the GitHub Actions release matrix.
