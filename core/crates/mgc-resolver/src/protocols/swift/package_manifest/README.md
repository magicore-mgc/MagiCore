# Swift package manifest parsers / Parser manifest Swift

This owner exposes stable Swift package parsing APIs while separating Package.swift text scanning from `swift package dump-package` JSON decoding. Requirement-editing helpers remain restricted to the Swift protocol module. The former nested `package_manifest::package_manifest::*` path remains a re-export-only compatibility facade.

Owner này giữ API parse package Swift ổn định và tách quét văn bản Package.swift khỏi giải mã JSON `swift package dump-package`. Helper chỉnh requirement vẫn giới hạn trong module Swift protocol. Đường dẫn lồng cũ `package_manifest::package_manifest::*` được giữ dưới dạng façade chỉ re-export.
