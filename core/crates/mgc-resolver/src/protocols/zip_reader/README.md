# ZIP reader owner

## English

Owns the shared minimal ZIP reader used by native resolver protocols and adapters. The public `protocols::zip_reader::{ZipEntry, read_zip_entries, extract_zip, crc32}` paths stay stable. The reader coordinator delegates EOCD, central/local records, CRC, path validation, extraction, DTO, and resource limits to named owners. It retains the current 256 MiB per-entry, 1 GiB archive-total, and 16,384-entry caps; store/deflate-only support; ZIP64 rejection; diagnostics; and error order. Source callers were mapped in Go, PyPI, Swift, and NuGet. Behavior tests live beside owners; the integration test keeps public-path smoke coverage. Executable checks are deferred until the repository-wide structural pass is complete.

## Tiếng Việt

Bộ đọc ZIP dùng chung cho các protocol resolver native và adapter. Giữ ổn định các đường dẫn public `protocols::zip_reader::{ZipEntry, read_zip_entries, extract_zip, crc32}`. Coordinator ủy quyền EOCD, central/local record, CRC, kiểm tra path, giải nén, DTO và giới hạn tài nguyên cho owner tương ứng. Giữ cap 256 MiB mỗi entry, 1 GiB tổng archive, 16.384 entry; chỉ hỗ trợ store/deflate; từ chối ZIP64; giữ diagnostic và thứ tự lỗi hiện có. Caller đã được đối chiếu ở Go, PyPI, Swift và NuGet. Test behavior đặt cạnh owner; integration test giữ smoke test public path. Chưa chạy kiểm tra executable cho tới khi hoàn tất structural pass toàn repo.
