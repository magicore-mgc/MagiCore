# CAS archive spool / Spool archive cho CAS

## Responsibility / Trách nhiệm

Streams bounded tar entries into a private temporary directory before importing them into the content store. Duplicate package paths keep the last regular-file entry.

## Tiếng Việt

Đọc tar có giới hạn vào thư mục tạm riêng trước khi import vào content store. Nếu archive lặp đường dẫn package, file thường xuất hiện cuối cùng được giữ lại.
