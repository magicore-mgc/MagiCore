# V3 serialization / Serialization v3

This owner serializes legacy `Lockfile` values and is the common product boundary for Web, App, Lib, CLI dedupe, and CLI import v3 writes. It validates the whole merged lock before TOML bytes are returned. A classified row needs a non-blank registry, imported-lockfile provenance naming its source file, or named delegated-tool provenance.

Owner này serialize `Lockfile` legacy và là boundary chung của writer v3 từ Web, App, Lib, CLI dedupe và CLI import. Nó xác minh toàn bộ lock trước khi trả TOML. Row đã phân loại cần registry khác rỗng, provenance lockfile import có tên file nguồn, hoặc provenance tool ủy thác có tên.

`registry-import` is accepted only with a non-blank `imported_from` value; the importer records the real source lockfile basename rather than inventing a registry URL. The compatibility exception for `EcosystemTag::Other` is deliberate for v2-era and unclassified imports. A classified row missing every complete source form returns `IncompleteProvenance`.

`registry-import` chỉ được chấp nhận khi `imported_from` khác rỗng; importer ghi basename lockfile thật, không bịa registry URL. Miễn trừ `EcosystemTag::Other` được giữ có chủ đích cho import v2 và dữ liệu chưa phân loại. Row đã phân loại thiếu mọi dạng source đầy đủ trả lỗi `IncompleteProvenance`.
