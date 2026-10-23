# Dangling symlink repair

## Purpose

Owns walking a materialized `node_modules` tree and repairing symlinks whose targets are missing, so a stale link cannot shadow a valid package on the next install.

## Vietnamese

Sở hữu việc duyệt cây `node_modules` đã materialize và sửa các symlink có target không còn tồn tại, để link cũ không che package hợp lệ ở lần cài sau.
