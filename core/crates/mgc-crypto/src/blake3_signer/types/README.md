# Digest types

Owns `Blake3Hash`, the 32-byte digest representation, display and encoding API. Public module paths stay compatible. Parsing failures map through the existing `CryptoError` surface. Add behavior tests under `test/` in this module tree; integration tests remain at crate boundary.
