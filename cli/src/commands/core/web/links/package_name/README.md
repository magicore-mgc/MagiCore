# Web package-link naming

This owner validates link target segments and preserves scoped package layout: `@scope/package` maps to `node_modules/@scope/package`. Explicit local paths still use their final path segment; unsafe traversal and platform-special filename segments are rejected.

# Quy tắc tên target link package Web

Owner này kiểm tra từng path segment và giữ cấu trúc scoped package: `@scope/package` ánh xạ thành `node_modules/@scope/package`. Path cục bộ tường minh vẫn dùng đoạn cuối làm target; traversal và segment chứa ký tự filename đặc biệt theo OS bị từ chối.
