# mgc-python-service

Shared native Python (PEP 621) service seed — hạt giống service Python native
dùng chung (slice S09a, decision D012 PROPOSED in
`docs/specs/wholeprojectrefactor/tracking/batches/S06-r07-remediation.md`).

## Responsibility / Trách nhiệm

Own the MGC-native Python lane's pure domain logic:

- `manifest`: PEP 621 ownership predicate
  (`supports_native_python_project`), PEP 735 `dev`-group gate, and
  `pyproject.toml` parse/write (`parse_pyproject_manifest`,
  `write_pyproject_manifest`, `parse_python_dependency`,
  `python_dependency_value`).
- `install`: stable API facade over the `distribution_name`, `external_manifest`,
  `core_owner`, `artifact_cache`, and `runtime_compatibility` responsibility
  owners. The owners separately handle PEP 503 comparison, foreign-manifest
  detection, core ownership, no-follow digest-verified cache reads, and Python
  path/wheel compatibility. Each fallible owner defines a local error and maps
  to the prior public `MgError` class/message.

## Non-goals (stay with current owners) / Không thuộc phạm vi

- Generic resolve→lock wiring (`resolve_with_protocol{,_including_dev}` in
  `adapters/lib/src/native/engine.rs`): shared by Rust/Go/Java/DotNet lanes
  and app Flutter/Swift callers — extraction deferred to the P04 wave
  (S06 D013), not moved for Python alone.
- `install_python_native` orchestration, `native_python_path_entries{,_from_store}`,
  `native_python_installed_packages`, `read_existing_lock` family: they depend
  on lib-owned install infra (`shared_store::SharedStoreRun`,
  `content_store`, lock readers) and call back into this crate's leaf helpers
  through same-name COMPAT facades in `adapters/lib`.

## Owner / Callers / Dependencies

- Owner: shared service crate. Current consumer: `mgc-lib-adapter` via
  one-line COMPAT facades (`// COMPAT facade (S09a/P03)` + caller list +
  `TODO(S09b-cleanup)`); indirect consumers unchanged: `mgc-ai-adapter`
  (`supports_native_python_project` via lib re-export, `LibAdapter` embedding
  intact) and `cli` (`native_python_path_entries`, body stays in lib).
- S09b still must do: retarget ai onto this crate, drop ai's
  `mgc-lib-adapter` dep, remove `LibAdapter.security_core`.
- Dependencies (minimal, exactly what moved code imports): `mgc-types`,
  `mgc-adapter-base`, `mgc-config`, `mgc-lockfile`, `mgc-resolver`, `toml`,
  `sha2`, `hex`, `libc` (unix `O_NOFOLLOW` in the cache read).
- Error surface: the public functions retain `mgc_types::MgResult`; each
  fallible install owner keeps a local typed error and maps it at the stable
  facade to the existing `MgError` class and display. Manifest parsing retains
  its existing local error owners.

## Tests / Evidence

- Nearest install-owner tests cover the behavior in `src/install/*/test/`;
  existing Lib tests remain in place to exercise compatibility facades.
- Verification performed by the implementer (no `cargo` run in this batch per
  slice constraints): single-definition grep, byte-identical body diff
  (modulo `fn` → `pub fn`), and complete caller-set grep — see the S09a batch
  report. `cargo test -p mgc-python-service` and the lib suite are UNVERIFIED
  until a later batch runs them.
