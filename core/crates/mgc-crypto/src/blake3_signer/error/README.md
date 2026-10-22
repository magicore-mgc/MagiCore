# BLAKE3 local errors

Owns typed encoding and digest-width failures. Public methods preserve the historical `CryptoResult` and `CryptoError::Blake3Failed` mapping for compatibility. I/O keeps its standard source chain through the existing `CryptoError::IoError` conversion.
