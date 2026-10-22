# Dependency name validation

This owner validates route/package-name segments before they can be joined to the project's `node_modules` path. It rejects backslash, colon, Windows reserved device names/characters, trailing dot/space, controls and JavaScript line separators on every OS, then validates plain/scoped package subpaths without allocating a segment vector.

Owner này kiểm tra segment tên route/package trước khi chúng được nối với path `node_modules` của project. Nó chặn backslash, colon, tên/ký tự device Windows, dấu chấm/khoảng trắng cuối, control và JavaScript line separator trên mọi OS, sau đó kiểm tra subpath package thường/scoped mà không cấp phát vector segment.
