# OCI route parsing

## Purpose

Owns splitting `/v2/{rest}` into `(name, section, tail)` with longest-match selection across `blobs/uploads`, `blobs`, `manifests` and `tags/list`, plus the per-section tail validity rules. This is the anti-confusion boundary for OCI routing.

## Vietnamese

Sở hữu việc tách `/v2/{rest}` thành `(name, section, tail)` với chọn match dài nhất qua `blobs/uploads`, `blobs`, `manifests`, `tags/list`, cùng quy tắc hợp lệ tail theo section. Đây là ranh giới chống nhầm route của OCI.
