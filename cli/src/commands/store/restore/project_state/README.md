# Restore project state / Project state khi restore

Captures the resolved database/vstore destinations and existing project state after the caller acquires the Store maintenance lock. It also owns rejection of a restore source that overlaps protected project-store roots. The command-level `RestoreError` remains shared because these checks are part of the restore CLI error boundary.

Chụp destination database/vstore cùng state hiện hữu sau khi caller lấy Store maintenance lock. Owner cũng từ chối restore source overlap với root Store được bảo vệ. `RestoreError` cấp command được dùng chung vì các lỗi này thuộc cùng biên lỗi CLI restore.
