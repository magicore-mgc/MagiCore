# audit

EN: AI model audit boundary for severity/finding/report values, file and directory routing, and format scanners.

VI: Ranh giới audit model AI cho severity/finding/report, điều phối file/thư mục và scanner định dạng.

## Owners

- `severity/`: ordered finding severity.
- `finding/`: public finding values and constructors.
- `audit_report/`: pass state and severity counters.
- `model_audit/`: public `audit_model` entry.
- `file_audit/`: extension-to-scanner routing.
- `directory_audit/`: recursive file traversal.
- `scanner.rs`: existing public scanner module, frozen by S275 because GitNexus reports CRITICAL/UNKNOWN impact; its own decomposition requires a separate narrow batch.

The public API paths and existing `MgError`/`MgResult` boundary stay unchanged. The current directory traversal still ignores individual `read_dir` errors and follows symlink directories; a separate security fix is required before product-readiness approval. S275 has not run tests/build/CLI/E2E/CI.
