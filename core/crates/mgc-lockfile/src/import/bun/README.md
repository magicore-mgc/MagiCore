# Bun lockfile parser

Parses text `bun.lock` JSONC, keeps the established version guard, extracts dependency names, and records entries that cannot map to the lock schema. Binary `.lockb` remains outside this parser's scope.

Parse JSONC của `bun.lock` dạng text, giữ nguyên chốt version, lấy tên dependency và ghi các entry không ánh xạ được sang schema. `.lockb` nhị phân nằm ngoài phạm vi parser này.

JSONC stripping uses the single shared `mgc-types::strip_jsonc` implementation.

Cắt JSONC bằng implementation dùng chung duy nhất `mgc-types::strip_jsonc`.
