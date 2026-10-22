# Default backup path / Path backup mặc định

Resolves `<user-home>/.magicore/store_backup_<nanosecond-timestamp>` through the platform home-directory helper and refuses an existing `.magicore` final symlink or non-directory. Resolving the path does not create directories; the tree owner creates them after destination validation.

Phân giải `<home-user>/.magicore/store_backup_<timestamp-nanosecond>` bằng helper home của nền tảng và từ chối `.magicore` hiện có nếu là symlink cuối hoặc không phải directory. Bước phân giải không tạo thư mục; tree owner tạo sau khi xác thực đích.
