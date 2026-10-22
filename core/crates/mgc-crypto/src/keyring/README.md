# Keyring owner

Owns Ed25519 key-pair lifecycle, keyring selection, secure persistence and their local typed policy errors. It preserves `mgc_crypto::keyring::{KeyPair, Keyring}` and the existing `CryptoError::KeyringFailed` boundary.

Sở hữu vòng đời key pair Ed25519, chọn key, lưu trữ bảo mật và lỗi policy có kiểu. Giữ `mgc_crypto::keyring::{KeyPair, Keyring}` cùng boundary `CryptoError::KeyringFailed` hiện có.

`mod.rs` only wires private owners and re-exports the public types. Storage helpers never decide trust authorization. Focused tests live beside owners; `tests/keyring.rs` remains the public API integration surface and calls production `Keyring::save`.

`mod.rs` chỉ nối owner private và re-export type public. Storage helper không quyết định trust authorization. Test tập trung nằm cạnh owner; `tests/keyring.rs` giữ contract integration public và gọi `Keyring::save` production.
