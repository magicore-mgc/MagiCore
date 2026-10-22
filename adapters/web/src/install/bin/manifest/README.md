# Installed manifest / Manifest package đã cài

Defines the public serde types for the `bin` field and maps a package manifest into `(binary_name, relative_target)` entries. Single-string `bin` names continue to derive from the last package-name segment; map entries retain the existing map iteration behavior.

Định nghĩa serde type public cho trường `bin` và chuyển manifest package thành `(binary_name, relative_target)`. `bin` dạng chuỗi tiếp tục suy ra tên từ segment cuối package; dạng map giữ nguyên cách lặp hiện tại.

Missing `package.json` still returns an empty list. Read errors remain `MgError::Io`; parse errors remain contextual `MgError::Other`. Add parser cases in the later nearest-owner test phase. `cargo test -p mgc-web-adapter` is documented for that phase and was not run in S397.

Thiếu `package.json` vẫn trả danh sách rỗng. Lỗi đọc vẫn là `MgError::Io`; lỗi parse vẫn là `MgError::Other` có context. Bổ sung case parser ở phase test gần owner sau. Lệnh `cargo test -p mgc-web-adapter` dành cho phase đó, chưa chạy trong S397.
