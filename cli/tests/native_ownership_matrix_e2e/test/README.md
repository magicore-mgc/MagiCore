# Native ownership test cases / Test native ownership

The original assertion bodies are grouped by canary, core/lifecycle, capability matrix, failure, and transaction behavior. All-core capability manifest tests have their own module, separate from Cloud-only lifecycle refusal cases. Module functions are called by top-level `#[test]` wrappers in the stable integration target, preserving exact Cargo harness names and the ignored Node probe. No workflow is changed.

Các thân assertion gốc được nhóm theo canary, core/lifecycle, capability matrix, failure và transaction. Test capability all-core có module riêng, tách khỏi ca từ chối lifecycle chỉ dành cho Cloud. Hàm module được gọi bởi wrapper `#[test]` cấp gốc của integration target, giữ đúng tên Cargo harness và trạng thái ignore của probe Node. Không đổi workflow.

All tests remain NOT_RUN in this source-first phase.
