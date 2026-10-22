# Sigstore attestation

Parses the registry-provided DSSE bundle, requires the in-toto statement media/type, binds subject SHA-256 and registry predicate fields to the signed payload, and applies the existing expected workflow identity + issuer policy through Sigstore.

Implementation: `sigstore_attestation.rs`; typed error mapping: `error.rs`; nearest statement tests: `test/sigstore_attestation.rs`. Production trust-root loading and verifier creation live here; orchestration caches the verifier for the whole page walk.
