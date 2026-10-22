# Environment and file token source

English: Owns the public environment variable names and the precedence helper: nonempty env token, then configured token file, then caller fallback to GitHub Actions.

Tiếng Việt: Sở hữu tên env công khai và helper thứ tự ưu tiên: token env không rỗng, file token được cấu hình, rồi caller fallback sang GitHub Actions.

Read failures keep the original generic/Sigstore diagnostics. Unit tests cover normalization; no process-global environment mutation is used.
