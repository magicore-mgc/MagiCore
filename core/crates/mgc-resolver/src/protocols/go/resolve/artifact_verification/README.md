# Go artifact verification

Owns fail-closed verification of downloaded zip bytes. A declared proxy sha256 takes precedence; otherwise the recorded sumdb h1 directory hash is used. Missing integrity evidence or any mismatch remains an integrity failure.

## Tiếng Việt

Folder này sở hữu việc xác minh bytes zip theo fail-closed. Sha256 proxy đã khai báo được ưu tiên; nếu không có thì dùng directory hash h1 sumdb đã ghi. Thiếu bằng chứng integrity hoặc bất kỳ hash nào không khớp đều tiếp tục là lỗi toàn vẹn.
