# Registry chain verification

Verifies the encoded public key's key ID, package/schema/sequence/previous-hash invariants, protocol-specific digest form, canonical payload hash, and Ed25519 signature. The page orchestrator pins one head sequence and passes the previous hash between pages.

Implementation: `registry_chain.rs`; typed error mapping: `error.rs`; nearest tests: `test/registry_chain.rs`. Signature and hash algorithms are unchanged.
