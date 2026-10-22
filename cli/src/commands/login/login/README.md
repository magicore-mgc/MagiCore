# Login coordinator

EN: This owner sequences registry selection, credential input, user registration and token persistence. It keeps the CLI success output at the command boundary.

VI: Owner này giữ thứ tự chọn registry, nhập credential, đăng ký user và lưu token. Dòng thông báo thành công vẫn thuộc ranh giới command.

`mod.rs` only wires the implementation and re-exports `run`. The implementation does not perform URL parsing, HTTP response decoding, password parsing or `.npmrc` writes itself; those responsibilities belong to sibling owners. The dispatcher signature stays `run(LoginArgs) -> anyhow::Result<()>`.

`mod.rs` chỉ nối implementation và export `run`. Implementation không tự parse URL, giải mã HTTP response, parse password hay ghi `.npmrc`; các phần đó thuộc owner riêng. Signature dispatcher giữ nguyên `run(LoginArgs) -> anyhow::Result<()>`.

The current integration contract is covered by `cli/tests/login_security_e2e.rs`; it is preserved and has not been executed in this source-first phase.
