# Link-safe project reads

## Purpose

Owns refusing link-swapped project paths and reading project files with `O_NOFOLLOW` on unix, so a planted symlink fails the open instead of redirecting a read outside the project.

## Vietnamese

Sở hữu việc từ chối path project bị tráo link và đọc file project với `O_NOFOLLOW` trên unix, để symlink bị cài sẽ làm open thất bại thay vì chuyển hướng đọc ra ngoài project.
