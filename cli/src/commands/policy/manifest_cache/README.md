# Signed manifest cache owner / Owner cache manifest đã ký

Own the 1 MiB limit, JSON parsing, signature verification, monotonic `issued_at` checks, revocation retention, cross-process writer lock and atomic cache replacement. `cache_lock/` owns the OS lock and `verified_cached_manifest/` reads a trusted prior snapshot. `error.rs` contains typed failures with sources; public refresh APIs retain their `anyhow::Result` signature.

Transaction order and messages are preserved. The fixed `.json.tmp` path remains a security follow-up because an attacker may pre-create a symlink in a writable cache directory; S207 does not claim that issue is fixed.

Nearest tests cover lock serialization, malformed cached input, typed diagnostic/source behavior and oversize rejection. Tests NOT_RUN.
