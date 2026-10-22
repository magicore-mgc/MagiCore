# pub cache materialization / Materialize cache pub

`materialize.rs` owns the path layout `{pub_cache}/hosted/pub.dev/{name}-{version}` and delegates archive path/link/size safety to the shared `protocols::archive::extract_tar_gz` owner. It does not verify `ResolvedEntry.sha256`; the Flutter installer verifies downloaded/cache bytes before calling it, and direct callers must provide bytes whose integrity they have checked. The public method does not alter extraction, cleanup or cache policy. `test/pub_cache_layout.rs` checks the destination and extracted file contents with an in-memory tar.gz fixture.

Archive path/link safety remains owned and tested by the shared archive module. Resolver integration coverage remains in `core/crates/mgc-resolver/tests/protocols_pub.rs`; runtime is NOT_VERIFIED because this source-only refactor has not run tests.
