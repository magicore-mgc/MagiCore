# Project-root discovery

## Purpose

Owns `find_project_root`: walk upward from a starting path until a project marker (`mgc.toml`, `mgc.lock`, `package.json`, `Cargo.toml`, `pyproject.toml`, `.git`) is present. Marker precedence and traversal are unchanged.

## Phạm vi

Owner này sở hữu `find_project_root`: đi ngược từ path bắt đầu tới khi gặp marker project (`mgc.toml`, `mgc.lock`, `package.json`, `Cargo.toml`, `pyproject.toml`, `.git`). Thứ tự marker và cách duyệt không đổi.
