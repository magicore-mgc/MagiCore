# AI development runtime

Owns the `mgc dev ai` setup and execution flow: freshness check, framework entry-script selection, optimizer environment, native Python environment, shared `ExecOptions`, and `mgc-exec` invocation. It does not launch a Python package manager. The packaged CLI runtime contract is covered by `cli/tests/native_python_runtime_e2e.rs`; this local unit test protects refusal before any child process starts.

# Runtime dev của AI

Sở hữu luồng chuẩn bị/chạy `mgc dev ai`: freshness, entry script framework, optimizer env, Python env native, `ExecOptions` dùng chung và gọi `mgc-exec`. Owner này không chạy Python package manager. Contract runtime CLI đóng gói nằm ở `cli/tests/native_python_runtime_e2e.rs`; unit test tại đây bảo vệ việc từ chối trước khi spawn process con.
