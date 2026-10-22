# Source and static route / Route source và file tĩnh

Owns wildcard project-file lookup, CSS/static responses, extension dispatch, and the handoff to the native transpile pipeline.

Sở hữu tra file project qua wildcard, response CSS/file tĩnh, dispatch theo extension và chuyển tiếp sang pipeline transpile native.

The request-path gate runs before joins; response status and cache headers are part of this owner's preserved HTTP contract.

Cổng request-path chạy trước phép nối; status và cache header thuộc HTTP contract được giữ nguyên.
