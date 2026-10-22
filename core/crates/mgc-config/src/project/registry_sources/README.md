# Project registry sources owner

This owner stores the serialized `[[sources]]` DTO used to describe registry/index claims and trust constraints. It does not select or contact a source; registry resolution owns those decisions. `SourceConfig` remains re-exported from `mgc_config::project`.

`id` and `url` remain required during deserialization. The other fields keep their current defaults, and no error type is added to a data-only DTO.

# Owner registry source của project

Owner này lưu DTO `[[sources]]` để mô tả claim và ràng buộc trust của registry/index. Nó không lựa chọn hay kết nối source; registry resolver sở hữu các quyết định đó. `SourceConfig` vẫn được re-export từ `mgc_config::project`.

Khi deserialize, `id` và `url` vẫn bắt buộc. Các field khác giữ default hiện tại; DTO thuần dữ liệu không cần error type riêng.
