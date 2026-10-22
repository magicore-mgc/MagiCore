# Script policy tests

Decision tests pin deny precedence across the committed file and machine trust DB, including file deny vs database approval. Parser tests pin fail-closed malformed TOML, the compatible diagnostic prefix, and a retained error source. Existing integration tests continue to cover full `mgc.toml` loading and adapter-facing behavior.

# Test policy script

Test quyết định khóa thứ tự deny giữa file đã commit và trust DB trên máy, gồm file deny thắng database approval. Test parser khóa fail-closed khi TOML lỗi, prefix diagnostic tương thích và việc giữ source error. Integration tests hiện tại tiếp tục bao phủ luồng load `mgc.toml` đầy đủ và hành vi phía adapter.
