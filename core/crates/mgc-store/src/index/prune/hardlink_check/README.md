# Hardlink count check

## Purpose

Owns deciding whether a cache file is exclusively owned (link count <= 1) so pruning never deletes a file another tree still references. Non-unix platforms return false (unknown, do not prune).

## Vietnamese

Sở hữu việc quyết định file cache có thuộc sở hữu duy nhất (link count <= 1) để prune không xoá file mà cây khác còn tham chiếu. Nền tảng không phải unix trả false (không rõ, không prune).
