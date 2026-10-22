# Audit runner / Bộ chạy audit

`runner.py` traverses the declared roots, combines findings, writes the ignored JSON ledger and returns the existing blocking exit code. The stable `audit.main()` supplies the active checkout root and mutable lane-gate callback.

`runner.py` quét roots, ghép finding, ghi JSON ledger bị gitignore và trả exit code blocking cũ. `audit.main()` truyền root checkout và callback lane gate có thể cấu hình trong test.
