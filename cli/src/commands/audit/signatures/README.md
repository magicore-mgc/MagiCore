# Signature audit owners

This folder preserves the public `commands::audit::signatures::run` API and assigns request coordination, package scope, bounded response decoding, registry chain verification, and Sigstore statement validation to separate owners.

The audit accepts HTTPS registries and loopback HTTP only, rejects redirects, bounds each page response, pins the page snapshot, verifies key ID + Ed25519 entry signatures + canonical SHA-256 chain, and checks public Sigstore identity when a public artifact attestation exists. Legacy registry-only records remain separately counted.

`mod.rs` is wiring/export only; `signatures.rs` owns request/page orchestration and `error.rs` owns typed coordinator failures. The `test/` folder covers CLI binding and coordinator diagnostics; each behavior owner registers tests in its own `test/` folder. Tests are authored but NOT_RUN in S353. No algorithm or trust policy is changed by this extraction.
