# Web installed-package listing

## Purpose

Owns `run_list`: reads the manifest, prefers the verified V4 graph (never parsed through the legacy reader), and reports installed packages only when the materialized `package.json` proves a version.

## Phạm vi

Sở hữu `run_list`: đọc manifest, ưu tiên graph V4 đã verify (không đi qua reader legacy), và chỉ báo package đã cài khi `package.json` đã materialize chứng minh được version.
