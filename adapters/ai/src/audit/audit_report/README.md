# audit_report

EN: Owns report state, finding insertion and Critical/High counters.

VI: Quản lý trạng thái report, thêm finding và bộ đếm Critical/High.

- API: `audit::AuditReport` remains public with unchanged fields/method signatures.
- Invariant: only a finding at High or Critical makes `passed` false; findings keep insertion order.
- Errors: in-memory report operations are infallible and use the existing `Finding`/`Severity` types.
- Extension: report aggregation semantics belong here; file traversal belongs to `model_audit`.
- Tests: `test/passed_state.rs` and `test/counts.rs`; not executed in this source-first pass.
