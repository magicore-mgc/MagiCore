# BLAKE3 digest module

Owns the stable BLAKE3 digest API while separating representations, hashing, file reads, encoding and local parse errors. Public paths `mgc_crypto::blake3_signer::{Blake3Hash, Blake3Hasher}` remain stable.

- `types/`: 32-byte digest value and display.
- `hash/`: byte/string digest and verification API.
- `file/stream/`: bounded-memory file hashing.
- `file/mmap/`: compatibility method name; implementation intentionally streams and makes no mmap performance claim.
- `encoding/{hex,base64}/`: bounded/standard codecs with nearest-owner tests.
- `error/`: local encoding failures mapped to the established crate error type.

Run the registered crate target with `cargo test -p mgc-crypto --test blake3_signer` and unit tests with `cargo test -p mgc-crypto --lib blake3_signer`. Public integration regressions remain in `tests/blake3_signer.rs`.
