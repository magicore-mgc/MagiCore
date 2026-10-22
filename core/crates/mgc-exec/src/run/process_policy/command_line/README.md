# command_line

Owns the existing whitespace-token command-line matcher and the name-only projection wrapper. Its behavior remains compatible with existing consumers; this structural move does not harden text matching.

Sở hữu matcher command-line hiện tại dùng token tách theo whitespace và wrapper chỉ trả tên. Giữ tương thích caller; lần tách này không thay đổi chính sách dò text.

This legacy path returns an optional match and performs no I/O. Focused tests live under test/.
