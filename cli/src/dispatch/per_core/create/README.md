# Per-core create adapter

This owner converts the nine explicit create CLI variants into their matching core commands. It preserves the distinct payloads: Web's `ScaffoldFlags`, four `ContainerFlags` values, and four framework/project-name pairs. Unmatched commands keep the `Option<CoreCommand>` fallback.

Owner này đổi chín variant CLI create tường minh thành core command tương ứng. Nó giữ các payload khác nhau: `ScaffoldFlags` của Web, bốn bộ `ContainerFlags`, và bốn cặp framework/project-name. Lệnh không khớp tiếp tục dùng fallback `Option<CoreCommand>`.

The mapper performs no fallible work, so it owns no local error type. Its nearest tests exercise all nine variants and verify payload preservation without invoking scaffold execution.

Mapper không có thao tác phát sinh lỗi nên không cần error type cục bộ. Test gần owner chạy qua đủ chín variant và xác minh payload được giữ nguyên mà không gọi scaffold execution.
