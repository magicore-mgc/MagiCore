# Flutter OSV audit

This owner checks for `pubspec.lock`, reads it with the shared bounded reader, returns local preflight reports, then delegates registry-backed advisories to the shared OSV client.

Owner này kiểm tra `pubspec.lock`, đọc bằng bounded reader dùng chung, trả report tiền kiểm cục bộ rồi giao advisory registry cho OSV client chung.
