# Dependency response route / Route response dependency

Owns the `/@magicore/deps` handler: cross-platform package-name validation, plain/scoped subpath URLs, shared DepsCache lookup, JavaScript response headers, CSS-as-data injection and safe encoding of a failed package name in returned JavaScript.

Sở hữu handler `/@magicore/deps`: kiểm tra tên/subpath package liên OS, tra DepsCache dùng chung, header JavaScript, chèn CSS dưới dạng dữ liệu và encode an toàn tên package khi bundle lỗi được trả dưới dạng JavaScript.

Pure response-payload serialization is in the adjacent `response_payload/` owner with local tests; real-server route behavior remains covered by CLI integration tests.

Serialize payload response thuần nằm trong owner `response_payload/` kế bên và có test local; behavior route qua server thật được test ở integration CLI.

Invalid names remain fail-closed with 403; invalid/out-of-package entry resolution produces the existing 404 response. Package entry files are constrained by the cache owner; esbuild transitive imports remain an explicit open boundary. This route does not execute a package manager.

Tên không hợp lệ tiếp tục bị chặn 403; entry không hợp lệ/nằm ngoài package vẫn dùng response 404 hiện tại. File entry được giới hạn ở cache owner; transitive import của esbuild còn boundary riêng đang mở. Route không chạy package manager.
