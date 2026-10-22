# Credential protection / Bảo vệ credential

Registry credential storage v1 uses salted Argon2id v19 (m=19456 KiB, t=2, p=1, 32-byte output) and domain-separated SHA-256 bearer digests. KDF work is limited to four blocking workers; permits survive request cancellation. HTTP uses async authorization while the synchronous library compatibility API remains available.

Credential registry v1 dùng Argon2id có salt riêng và digest bearer tách domain. HTTP đi qua authorization async; helper thư viện đồng bộ giữ tương thích. ProtectedUser phân biệt input plaintext với record đã hash. Không suy đoán plaintext bằng prefix.

Tests live in test/. Database upgrade/rollback/restart tests belong to storage/credential_migration/test/. No test or release PASS is implied by this document.
