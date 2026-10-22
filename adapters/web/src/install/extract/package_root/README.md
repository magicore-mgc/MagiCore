# Package root / Root package

Owns path- and byte-backed entry points that locate the extracted package and route through CAS claims, warm-cache verification and marker-backed materialization.

Quản lý entry point từ path/bytes, tìm package đã bung và điều phối claim CAS, kiểm warm-cache cùng materialize có marker.

Cache-hit callers can pass the already verified file handle so marker construction and extraction remain attached to the same opened file even if its cache path is later removed or replaced.

Caller cache-hit có thể truyền file handle đã xác minh để việc tạo marker và extraction vẫn gắn với cùng file đã mở kể cả khi cache path bị xóa hoặc thay sau đó.
