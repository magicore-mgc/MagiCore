# NuGet listed-version selection owner

Fetches the NuGet flat-container version index and chooses the lowest parseable listed candidate matching the requested range. Malformed version strings are skipped; an empty match is a fail-closed resolver error. The shared `MgError` contract is preserved.

## Tiếng Việt

Tải index version flat-container NuGet và chọn candidate listed thấp nhất parse được, khớp range. Chuỗi version lỗi parse được bỏ qua; không có kết quả thì resolver fail-closed bằng contract `MgError` dùng chung.
