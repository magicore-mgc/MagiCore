# Atomic replacement owner

Owns the platform replacement primitive used after a synced temporary file is complete. Windows retains `MoveFileExW` replacement/write-through flags; non-Windows targets retain `fs::rename`.

Sở hữu primitive replace theo OS sau khi temp file đã sync xong. Windows giữ flag replace/write-through của `MoveFileExW`; OS khác giữ `fs::rename`.
