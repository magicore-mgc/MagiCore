# Detected project owner / Owner project được nhận diện

`detected_project.rs` resolves a core from supported project signatures, verifies an explicit selector against that detected core, and records the initial `.mgc.core` ownership marker only after the check passes.

`detected_project.rs` tìm core từ chữ ký project được hỗ trợ, đối chiếu selector tường minh với core nhận diện, rồi chỉ ghi `.mgc.core` sau khi kiểm tra thành công.

Nearest tests cover conflicting and matching selectors plus nested-cwd discovery. Test source is not yet run.

Test gần owner kiểm tra selector xung đột/trùng khớp và tìm project từ cwd lồng nhau. Test chưa chạy.
