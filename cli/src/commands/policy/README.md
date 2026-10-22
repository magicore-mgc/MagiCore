# CLI policy owners / Owner policy CLI

Facade cho policy manifest có chữ ký. Giữ đường dẫn `commands::policy::*`, điều phối lane context, đọc manifest, cache file, HTTPS và OCI. Manifest chỉ được áp dụng khi trust root, signature, lane và profile gắn lock hợp lệ; refresh lỗi giữ fallback và native-deny như trước.

This facade preserves `commands::policy::*` routes and coordinates lane context, manifest loading, file cache, HTTPS and OCI channels. Signed exceptions require trusted keys, a valid signature, matching lane and a lock-bound profile.

Implementations live in `policy.rs`, `context/`, `manifest_load/`, `manifest_cache/`, `manifest_distribution/`, `manifest_url/` and `manifest_oci/`. Narrow subowners isolate manifest candidate paths, user trust keys, cache lock/snapshot reads, OCI reference parsing and artifact validation. `mod.rs` only wires and explicitly re-exports. Do not add policy decisions to the facade.

Public behavior contracts remain in `cli/tests/policy_manifest_load.rs`; focused tests live nearest to the context, key parser, cache, URL and OCI owners. S207 test source is written but NOT_RUN under the user's source-first order.

Security follow-ups remain open: URL and OCI bodies are buffered before the 1 MiB check, URL redirects are not checked for downgrade, and OCI loopback selection uses a string prefix. See the S207 tracking batch.
