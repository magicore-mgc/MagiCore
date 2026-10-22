# Streaming file digest

Reads fixed-size chunks and preserves the `std::io::Error` cause through `CryptoResult`. Memory use is bounded by the stack buffer. Tests are in `test/`.
