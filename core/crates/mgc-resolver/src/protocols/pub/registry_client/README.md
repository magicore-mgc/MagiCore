# pub.dev registry client / Client registry pub.dev

`registry_client.rs` owns package metadata and archive HTTP requests and their established `MgError` rendering. It uses the shared `mgc_http::HttpClient`; it does not add retries, alter endpoint policy or expose a new public API. The `package_doc` and `download_entry` methods remain internal to `protocols::pubdev` for the existing resolver and trait implementation. `test/package_doc.rs` verifies invalid package names fail before any HTTP request; network request/response behavior remains covered by the registered resolver integration test.

Network contract tests remain at `core/crates/mgc-resolver/tests/protocols_pub.rs`; runtime is NOT_VERIFIED in this structural phase.
