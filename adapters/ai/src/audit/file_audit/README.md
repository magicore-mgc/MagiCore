# file_audit

EN: Maps model file extensions to the existing pickle, safetensors and weight scanners.

VI: Ánh xạ phần mở rộng file model sang scanner pickle, safetensors và weights hiện có.

- Inputs/outputs: file path and mutable `AuditReport` → incremented scan count and appended findings.
- Invariant: extension routing and unknown-format skip behavior remain unchanged.
- Errors: scanner errors propagate through the existing `MgResult`.
- Extension: add a format only with a corresponding scanner and behavior tests.
- Tests: `test/single_file.rs` checks routing through the public audit entry. Scanner implementation is frozen in S275.
