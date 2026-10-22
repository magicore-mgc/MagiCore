# node

Owns Node script-operand parsing, including known value flags, switches, the option terminator and fail-safe handling of unknown options. Inline evaluation/check source is opaque and is never searched for package-manager text.

Sở hữu parse script operand của Node, gồm value flag, switch, dấu kết thúc option và dừng an toàn khi gặp option lạ. Source của chế độ eval/check luôn opaque và không bị tìm tên package manager.

Parser returns None when it cannot safely identify a file operand. It has no error type because no-match is expected. Nearest tests are in test/node.rs.
