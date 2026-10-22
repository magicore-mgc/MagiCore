# Package reference parsing

Owns the shared parsing of `name@version` package references, including scoped npm names, plus the pnpm peer-suffix trim.

Sở hữu cách parse chung `name@version`, gồm tên scoped của npm, và việc cắt hậu tố peer của pnpm.

The helpers remain internal to this importer. They do not normalize package versions or alter source strings beyond the transformations already required by each parser.

Các helper vẫn nội bộ importer. Chúng không chuẩn hóa version và không thay chuỗi nguồn ngoài biến đổi vốn có của từng parser.
