# Go version selection

Owns exact-pin recognition, the direct `.info` probe, version-list range matching, and the `@latest` fallback. Preserve the current fast-path and fallback order, pseudo-version exclusions, and error behavior when adding selection rules. The existing `GoModProtocol::exact_pin_version` visibility remains confined to the Go protocol.

## Tiếng Việt

Folder này sở hữu nhận diện exact-pin, probe `.info`, chọn version từ list theo range và fallback `@latest`. Khi thêm quy tắc chọn version phải giữ thứ tự fast-path/fallback, quy tắc loại pseudo-version và hành vi lỗi hiện tại. Visibility của `GoModProtocol::exact_pin_version` vẫn giới hạn trong Go protocol. Test gần owner nằm trong `test/`; full HTTP/protocol contract tiếp tục ở `core/crates/mgc-resolver/tests/protocols_go.rs`.
