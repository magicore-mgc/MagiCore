# V4 structural verification / Xác minh cấu trúc V4

Checks root references before recomputing the canonical payload digest and validating inline Ed25519 metadata. Returns `V4VerifyReport`; it does not decide whether a key is trusted. Root matching is isolated in `root_reference/`.

Kiểm tra root reference rồi tính lại digest và kiểm tra metadata Ed25519. Trả report cấu trúc, không quyết định khóa có trusted không. Phần đối chiếu root ở `root_reference/`.
