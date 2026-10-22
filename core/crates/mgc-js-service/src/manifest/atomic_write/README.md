# Atomic write / Ghi atomic

Owns safe replacement of regular files and atomic publication for missing paths. It uses the existing same-directory staging protocol and delegates replacement semantics to `mgc-lockfile`.

Sở hữu thay file thường an toàn và publish atomic khi đường dẫn chưa có. Module dùng protocol staging cùng thư mục hiện tại và giao semantics thay thế cho `mgc-lockfile`.

Final symlinks and non-regular destinations are rejected. On Unix, staging keeps mode `0644`, uses `O_NOFOLLOW`, syncs file and parent directory, and removes staging on failure.

Symlink cuối và đích non-regular bị từ chối. Trên Unix, staging giữ mode `0644`, dùng `O_NOFOLLOW`, sync file và thư mục cha, rồi xóa staging khi lỗi.
