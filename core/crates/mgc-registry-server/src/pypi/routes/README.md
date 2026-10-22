# PyPI route registration

Owns only the four existing `/pypi` route patterns and binds them to their local handler owners. Route behavior is exercised through the existing registry unit/integration tests; this owner has no fallible logic and therefore no `error.rs`.

Chỉ sở hữu bốn pattern `/pypi` hiện có và nối tới handler owner tương ứng. Hành vi route được kiểm tra qua unit/integration tests của registry; owner này không có logic fallible nên không tạo `error.rs`.
