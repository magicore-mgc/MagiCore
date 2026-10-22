# Destination directory validation

Owns contained child-directory creation and the empty-destination check. Symlink/non-directory refusal, canonical containment and error variants stay shared with backup/restore callers.

# Xác thực directory đích

Chịu trách nhiệm tạo directory con nằm trong root và kiểm tra destination rỗng. Từ chối symlink/non-directory, kiểm tra canonical containment và giữ variant lỗi dùng chung cho backup/restore.
