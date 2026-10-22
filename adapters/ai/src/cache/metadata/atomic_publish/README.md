# Metadata atomic publication

This owner writes a unique same-directory staging file with create-new semantics, syncs it, then calls the shared OS-aware atomic replace primitive. It rejects symlink/special destinations, applies permissions represented by `std::fs::Permissions` from an existing regular file, creates new Unix sidecars owner-only, and removes owned staging on error. Concurrent saves publish complete JSON; the last successful replacement wins. Custom Windows destination ACL preservation is not claimed; parent path components are not pinned, and a parent-sync error can occur after replacement is visible.

Owner này ghi staging name duy nhất cùng thư mục bằng `create_new`, sync rồi gọi primitive atomic replace theo OS. Từ chối đích symlink/file đặc biệt, áp dụng permission mà `std::fs::Permissions` biểu diễn từ file regular cũ, tạo sidecar Unix mới chỉ owner truy cập và chỉ dọn staging do chính writer tạo khi lỗi. Save đồng thời chỉ publish JSON hoàn chỉnh; lần replace thành công cuối cùng thắng. Không claim giữ custom Windows ACL; path cha chưa được ghim, và lỗi sync thư mục cha có thể xảy ra sau khi replace đã hiện diện.

Nearest behavior cases are in `test/atomic_publish.rs`.

Test hành vi gần owner nằm ở `test/atomic_publish.rs`.
