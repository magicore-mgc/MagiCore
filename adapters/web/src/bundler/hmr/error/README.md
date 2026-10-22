# HMR watcher errors

This owner wraps notify failures with a local error type while preserving the notify message and source chain through the public `anyhow::Result` boundary.

Owner này bọc lỗi notify bằng type local nhưng giữ nguyên message và source chain qua boundary public `anyhow::Result`.
