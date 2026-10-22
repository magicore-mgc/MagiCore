# file access / file access

Owns safe shared project-file operations and keeps their existing parent-visible names. The bounded no-follow text reader, config metadata guard, TOML parser, ecosystem composition and atomic writer each have a named owner and adjacent tests. `FileAccessError` preserves existing anyhow messages and source errors at the compatibility boundary.

Quản lý thao tác file project an toàn và giữ nguyên tên mà parent đang dùng. Bộ đọc text no-follow có giới hạn, guard metadata config, parser TOML, composition ecosystem và atomic writer đều có owner cùng test gần nhất. `FileAccessError` giữ nguyên anyhow message và source lỗi tại biên tương thích.
