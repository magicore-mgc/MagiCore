# Invocation policy

This owner runs the pre-spawn policy sequence: dependency-mutation rejection, exact MGC self-answered probes, execution-scope allowlisting, verified manifest exceptions, compat-runner recognition, and script-interpreter screening. `mod.rs` exports the internal facade; `invocation_policy.rs` preserves the decision order and result shapes; `test/` holds adjacent unit contracts. The implementation intentionally delegates dependency and script checks to their existing owners and leaves process-tree monitoring in the runner.

No local error or type module is needed in this owner: errors continue through the shared `anyhow::Result` boundary and request/report types remain owned by `run/types`. Verification has not run in this structural pass.

# Chính sách invocation

Owner này chạy chuỗi policy trước spawn: chặn mutation dependency, trả probe MGC tự xử lý, allowlist theo execution scope, exception manifest đã xác minh, nhận diện compat-runner và kiểm tra interpreter của script. `mod.rs` export facade nội bộ; `invocation_policy.rs` giữ nguyên thứ tự quyết định và dạng kết quả; `test/` chứa contract unit-test gần code. Implementation ủy quyền kiểm tra dependency và script cho owner tương ứng, còn monitor cây process vẫn thuộc runner.

Owner không cần `error` hay `types` riêng: lỗi tiếp tục đi qua biên `anyhow::Result` dùng chung, còn request/report thuộc `run/types`. Chưa chạy verification trong lượt refactor cấu trúc này.
