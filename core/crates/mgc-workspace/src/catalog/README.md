# Workspace catalogs / Catalog workspace

## English

This folder exposes the existing WorkspaceCatalogs data type, TOML loader and catalog: specifier resolver. The code is public through the current crate re-exports, but repository source search found no internal production call connecting it to add/install. The extraction does not claim end-to-end package lifecycle integration. The loader currently follows a symlink at magicore.workspace.toml; that safety gap remains open for a separate behavior fix.

## Tiếng Việt

Folder này cung cấp type WorkspaceCatalogs, bộ nạp TOML và resolver specifier catalog: hiện có. Code được public qua crate re-export, nhưng tìm kiếm source không thấy caller production nội bộ nối nó vào luồng add/install. Lát tách này không khẳng định đã tích hợp vòng đời package end-to-end. Loader hiện theo symlink ở magicore.workspace.toml; gap an toàn này còn mở cho batch sửa hành vi riêng.
