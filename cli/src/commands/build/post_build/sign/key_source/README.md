# key_source

EN: Single source of truth for the key `mgc build` signs with. Precedence: `MGC_SIGNING_KEYRING` (either keyring shape) → the `mgc trust` keyring's default key (`~/.magicore/keys/keyring.json`) → the legacy data-dir `keyring.json` with a `signing_key` seed (still honoured, warned) → no key. An unreadable or keyless trust keyring fails closed instead of silently signing with a different key, and a second, different keyring is reported as a conflict so a verifier's pinned key stays explainable.

VI: Nguồn chân lý duy nhất cho khoá `mgc build` dùng để ký. Thứ tự: `MGC_SIGNING_KEYRING` (một trong hai dạng keyring) → khoá mặc định của keyring `mgc trust` → keyring `keyring.json` kiểu cũ trong data dir với seed `signing_key` (vẫn dùng, có cảnh báo) → không có khoá. Keyring trust không đọc được hoặc không có khoá thì fail-closed, không lặng lẽ ký bằng khoá khác; keyring thứ hai khác nhau được báo là xung đột để khoá pin của verifier vẫn giải thích được.
