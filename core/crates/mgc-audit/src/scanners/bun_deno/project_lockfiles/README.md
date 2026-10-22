# Project Bun/Deno lockfile discovery

English: Uses `read_bounded_project_input` for each present `bun.lock` and `deno.lock`, then merges the results in that order. The helper does not read `mgc.lock` and source search found no production caller; it remains public for compatibility. File-input failures are propagated unchanged from the shared bounded reader.

Tiếng Việt: Dùng `read_bounded_project_input` cho từng file `bun.lock` và `deno.lock` đang có, rồi gộp kết quả theo thứ tự đó. Helper không đọc `mgc.lock`; source search chưa tìm thấy caller production và API vẫn được giữ. Lỗi đọc file được propagate nguyên trạng từ bounded reader dùng chung.
