# Atomic project-file writer / Ghi file project atomic

Creates a unique same-directory staging file, syncs file data, atomically replaces the target, syncs the parent on Unix, and cleans up failed staging files. It is shared by lock publication and verified archive cache writes.

Tạo staging file duy nhất cùng thư mục, sync dữ liệu, thay target atomic, sync thư mục cha trên Unix và dọn staging nếu lỗi. Dùng chung cho ghi lock và cache archive đã xác minh.
