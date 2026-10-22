# Integrity file operations / Thao tác integrity trên file

Owns file verification and computation. Verification intentionally retains `std::fs::read`; computation delegates to the existing BLAKE3 streaming API.

Sở hữu verify và compute trên file. Verify chủ đích giữ `std::fs::read`; compute gọi API BLAKE3 streaming hiện có.
