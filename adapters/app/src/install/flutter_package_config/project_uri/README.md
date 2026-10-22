# Flutter project URI

Owns conversion of the project directory into the URI format expected by Dart. Relative roots require a known current directory; Windows 8.3 paths are expanded before encoding.

`error.rs` distinguishes a missing current directory, URI-encoding failure, and Windows path-expansion failures. The facade maps these to the existing `MgError::Other` text; tests are in `test/project_uri.rs`.

## Chủ sở hữu

Sở hữu chuyển đổi thư mục project sang URI Dart mong đợi. Root tương đối cần current directory đã biết; Windows bung đường dẫn 8.3 trước khi encode.

`error.rs` phân loại thiếu current directory, URI không encode được và lỗi mở rộng path Windows; facade giữ nguyên `MgResult<String>`. Test nằm tại `test/project_uri.rs`.
