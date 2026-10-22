# wheel_record / wheel_record

Owns the stable `PypiProtocol::verify_runtime_materialization` entry and wires archive indexing, RECORD parsing, safe installed paths, streamed member checks and runtime tree comparison. Each behavior lives under its own same-name owner folder. The public digest preflight has a focused test under `test/`; the authenticated-wheel regression test is under `runtime_verification/test/`; parser/path/archive/member tests live beside their owners. The single test-only wheel fixture is shared through `test_support/`.

Sở hữu entry ổn định `PypiProtocol::verify_runtime_materialization` và nối index archive, parse RECORD, path cài an toàn, kiểm tra member dạng stream và đối chiếu cây runtime. Mỗi hành vi có owner folder cùng tên. Test preflight digest của entry công khai nằm tại `test/`; regression test authenticated-wheel ở `runtime_verification/test/`; test parser/path/archive/member ở cạnh owner. Fixture wheel test-only có một bản dùng chung trong `test_support/`.

Member file được mở bằng `mgc_platform::open_regular_file_no_follow` rồi so/hash qua buffer 64 KiB, tránh cấp phát thêm vector nguyên file. Bộ ZIP entry đã giải nén vẫn nằm trong RAM; component thư mục cha chưa được ghim bằng handle, nên race ở parent path vẫn là giới hạn còn lại.

Installed members open with `mgc_platform::open_regular_file_no_follow` and are compared/hashed through a 64-KiB buffer, avoiding another whole-file vector. The decompressed ZIP entries still reside in memory; parent directories are not pinned by handles, so parent-path races remain a limitation.
