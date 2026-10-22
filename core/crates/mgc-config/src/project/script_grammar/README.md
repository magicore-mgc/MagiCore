# Script grammar owner

This owner defines the optional serialized `[script].grammar` pin and the stable default grammar ID for each ecosystem family. `project.rs` keeps the effective pin-first resolver and re-exports the public type/function from the established `mgc_config::project::*` paths.

Grammar IDs are versioned contracts: this module only maps defaults and stores the pin. It does not parse scripts, decide whether a command is allowed, or make a script policy verdict. The operation is pure and infallible, so no local error type is needed.

# Owner grammar script

Owner này khai báo pin `[script].grammar` có thể vắng mặt và grammar ID mặc định ổn định theo họ ecosystem. `project.rs` giữ resolver ưu tiên pin tường minh và re-export type/function public tại path `mgc_config::project::*` hiện có.

Grammar ID là contract có version: module này chỉ map default và lưu pin. Nó không parse script, quyết định lệnh được phép hay tạo policy verdict. Operation thuần, không lỗi nên không cần type lỗi cục bộ.
