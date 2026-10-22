# `mgc-js-service` — Shared JS/npm Service Seed

Seed crate (S07; decisions D014, authority A05/P03/R07). One owner for the
core-independent npm/JS plumbing shared by Web (today) and App + Cloud
(after the S08 migration). No adapter-to-adapter edges; adapters consume
this crate, never each other.

## Responsibility

- `src/manifest/` — stable shared `manifest` API facade plus same-name owners
  for package.json conversion, `PackageJson` schema/IO, atomic write,
  write-if-changed, workspace-protocol recognition and local typed errors.
  Existing size guard and workspace skip/preserve policy remain unchanged.
- `src/registry.rs` — npm registry endpoint policy: `DEFAULT_NPM_REGISTRY`
  (value unchanged — R09/R14 compatibility surface), `effective_registry_url`,
  `validate_registry_allowed`, `allow_insecure_loopback_url` (loopback-http
  opt-in gate).
- `src/audit.rs` — pure npm Bulk Advisory primitives: tarball-trust
  predicate, advisory endpoint builder, bulk request shaper, fail-closed
  bulk-response parser. The `run_audit` runner stays in
  `mgc-web-adapter::audit` until S08 (lockfile-seam dependency, see Limits).
- `src/error.rs` — crate-level error-ownership index. Manifest-specific typed
  failures live beside manifest behavior in `src/manifest/error/`; registry
  and audit retain their existing error contracts.

## API

| Item | Signature (unchanged from web) |
|---|---|
| `manifest::parse_manifest` | `(project_root: &Path) -> MgResult<Manifest>` |
| `manifest::write_manifest` | `(project_root: &Path, manifest: &Manifest) -> MgResult<()>` |
| `registry::DEFAULT_NPM_REGISTRY` | `&str = "https://registry.npmjs.org"` |
| `registry::effective_registry_url` | `(default: &str) -> anyhow::Result<String>` |
| `registry::validate_registry_allowed` | `(url: &str) -> anyhow::Result<()>` |
| `audit::parse_advisory_bulk_response` | `(&Value, &[(String, String)]) -> MgResult<Vec<Vulnerability>>` |
| `audit::advisory_bulk_request_body` | `(&[(String, String)]) -> Map<String, Value>` |

Web keeps byte-identical behavior through one-line `pub use` compat facades
(`adapters/web/src/{manifest,audit,registry_config}.rs`, each marked
`COMPAT facade (S07/P03)` with caller list + `TODO(S10-cleanup)`).

## Config

Env vars honored (unchanged semantics):

| Var | Effect |
|---|---|
| `MAGICORE_WEB_REGISTRY_URL` | Overrides the default registry (must be `https://` unless loopback opt-in) |
| `MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST=1` | Allows `http://` loopback registries (tests / local mirrors); default deny |
| `MAGICORE_WEB_ALLOWED_REGISTRIES` | Comma allowlist; non-empty + no match = fail-closed rejection |
| `MAGICORE_WEB_ALLOWED_TARBALL_HOSTS` | Extra tarball hosts trusted beyond the registry host / npmjs CDN |

## Errors

Manifest policy and atomic-write failures use a local typed error owner, mapped
to the existing `MgResult` / `anyhow::Error` boundaries with stable display
text; workspace-source replacement remains the existing structured
`MgError::Unsupported`. IO and JSON errors retain their source chains. Registry
continues to use `anyhow::Error` fail-closed rejections (P0-4); audit continues
to use `MgError` contract violations and `String` per-entry schema reasons.

## Tests

- Manifest behavior has nearest-owner tests under `src/manifest/**/test/`;
  `tests/manifest.rs` remains the external public-route integration proof.
  Web adapter behavior continues through the Web facade; audit parsing remains
  covered by `adapters/web/src/test/audit_lane_test.rs`.
- Tests, compiler, formatter, CI and OS lanes are NOT_RUN in source-first S440;
  they remain unverified until the deferred verification phase:
  `cargo test -p mgc-js-service`, `cargo test -p mgc-web-adapter`.

## Limits

- `run_audit` / `run_audit_fix` / v4+legacy pin sourcing stay in web (S08
  seam work, needs App + Cloud retarget design D014/D015).
- `WebAdapter` construction + resolve/install engine untouched (migrate last, S10).
- Pre-existing same-valued `DEFAULT_NPM_REGISTRY` duplicate in
  `mgc-config::chain` is out of scope (dedup = central-edit task).
- Crate wiring (`[workspace.dependencies]` root entry) is a central edit;
  `mgc-web-adapter` consumes this crate via a direct path dep (existing
  workspace pattern) until then.
