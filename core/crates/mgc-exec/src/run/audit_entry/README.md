# Audit entry mapping

This owner converts an `ExecReport` into the stable `AuditEntry` persisted by the process runner. It owns the single argv-redaction boundary for native spawn results and MGC self-answered probes; the crate-level `audit` module owns serialization and file append. `mod.rs` exports the run-local adapter, `audit_entry.rs` performs the mapping, and `test/` pins field preservation and secret redaction. It defines no local type or error because it reuses `AuditEntry` and the existing append result boundary.

## Ánh xạ audit entry

Owner này chuyển `ExecReport` thành `AuditEntry` ổn định được process runner ghi lại. Nó sở hữu một ranh giới redact argv dùng chung cho kết quả spawn native và probe do MGC tự trả lời; module `audit` cấp crate sở hữu serialize và append file. `mod.rs` export adapter cục bộ, `audit_entry.rs` ánh xạ field, `test/` khóa việc giữ field và che secret. Không có type/error cục bộ vì dùng `AuditEntry` và biên kết quả append hiện có.
