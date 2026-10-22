# Runtime invocation

This folder recognizes executable entrypoints from argv vectors so the shared executor can apply process and dependency-resolution policy. The outer mod.rs preserves the existing run-level helper routes; runtime_invocation.rs is the dispatch coordinator; runtime-specific parsing lives in named owners.

Thư mục này nhận diện executable entrypoint từ argv để executor dùng chung áp policy process và phân giải dependency. mod.rs ngoài cùng giữ nguyên các helper route của run; runtime_invocation.rs điều phối; parser riêng nằm trong owner theo runtime.

## Owners

- argv_text: strict UTF-8 lookup for an argv position.
- node, python, php, shell, dart_pub and windows_cmd: runtime-specific entrypoint syntax.
- forbidden_entry: maps a discovered script/module to the shared forbidden-tool set.
- process_invocation: normalizes a discovered script token to its process identity.

## Contract

All parsers return Option because unsupported, ambiguous, or inline-code invocations are expected non-matches. Inline source is never scanned. Windows cmd only yields its first executable operand. This layer does not spawn processes, make policy exceptions, or change dependency decisions. No artificial local error type is used for ordinary no-match results.

Mọi parser trả Option vì cú pháp chưa hỗ trợ, mơ hồ hoặc code inline là trường hợp không match bình thường. Không parser nào quét source inline. Windows cmd chỉ trả executable operand đầu. Tầng này không spawn process, tạo ngoại lệ policy hay thay đổi quyết định dependency. Không tạo error type giả cho trường hợp no-match bình thường.

## Tests

The existing aggregate runtime test file remains under test/runtime_invocation.rs with its four original cases. Each parser/normalizer owner also has focused tests in its own test/ folder. These tests are authored only; runtime execution is deferred by the source-first phase and storage limit.
