# Audit rules / Quy tắc audit

`rules.py` aggregates common policy, `rust.py` process-boundary signatures, and language-specific rule owners. Literal fingerprints, roots, sentinels and review tables are security-reviewed data. Do not add executable scanning behavior here. Python native import/regex errors are allowed to surface at module import; an invalid rule must fail the gate early.

`rules.py` gom policy chung, `rust.py` giữ signature process-boundary Rust, cùng rule từng ngôn ngữ. Fingerprint, root, sentinel và review table là dữ liệu security-reviewed. Không đặt logic quét executable tại đây. Lỗi import/regex không bị nuốt; rule sai phải làm gate lỗi ngay.
