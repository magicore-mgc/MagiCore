# Ed25519 owner / Owner Ed25519

This module owns the stable Ed25519 API and delegates primitive operations to `ring`. It coordinates signer, signature, public-key, encoding, verification and local error owners. No custom cryptographic primitive or unsafe code belongs here.

Module này sở hữu API Ed25519 ổn định và ủy thác primitive cho `ring`. Nó nối owner signer, signature, public key, encoding, verification và lỗi local. Không đặt thuật toán mật mã tự viết hay unsafe code tại đây.

- Public compatibility paths remain `mgc_crypto::ed25519_signer::*` and crate-root type re-exports. / Public path tương thích giữ nguyên `mgc_crypto::ed25519_signer::*` và type re-export ở crate root.
- `Ed25519Signer` owns key-pair operations; `signature/` and `public_key/` own their distinct serialized values; `verification/` owns signature checking. / `Ed25519Signer` sở hữu thao tác key pair; `signature/` và `public_key/` sở hữu value serialize riêng; `verification/` sở hữu kiểm tra chữ ký.
- `error/` maps typed local failures to the established public `CryptoError` variants and messages. / `error/` ánh xạ lỗi local có kiểu sang variant/message `CryptoError` public đã có.
- Use the nearest owner tests plus the crate integration target. Runtime checks remain deferred until repository source closure. / Dùng test gần owner và integration target của crate. Runtime check chờ tới khi đóng source toàn repo.
