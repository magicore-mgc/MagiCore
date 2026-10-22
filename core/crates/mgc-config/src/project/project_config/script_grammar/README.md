# ProjectConfig script grammar / Script grammar ProjectConfig

EN: Owns the `ProjectConfig::script_grammar` accessor. An explicit pin wins; when absent, it delegates to the stable ecosystem default. Unknown pins are returned verbatim for the caller's existing fail-closed handling. `script_grammar.rs` implements the method; `mod.rs` only wires it; `test/` covers pin/default behavior.

VI: Sở hữu accessor `ProjectConfig::script_grammar`. Pin tường minh được ưu tiên; nếu thiếu thì gọi default ổn định theo ecosystem. Pin lạ được trả nguyên để caller fail-closed như hiện tại. `script_grammar.rs` triển khai method; `mod.rs` chỉ nối; `test/` kiểm tra pin/default.

No grammar policy or validation changes are in scope. Tests remain `NOT_RUN`.

Không đổi policy hoặc validation grammar trong scope này. Test vẫn `NOT_RUN`.
