# manifest / Manifest

Owns exact release/artifact binding and delegates detached signature checks to a child owner.
Sở hữu ràng buộc release/artifact và giao xác minh chữ ký rời cho owner con.

- `manifest.rs` — release version, target coordinates, size and digest.
- `signature/` — Ed25519 trust-root verification.
