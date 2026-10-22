# Cargo manifest errors

EN: Defines the local typed error taxonomy and the single conversion back to `MgError`.

VI: Khai báo taxonomy lỗi có kiểu tại chỗ và điểm chuyển duy nhất về `MgError`.

`ProjectFile` passes the shared project-file error through unchanged. TOML parse and version failures keep a source chain in the shared `Anyhow` carrier; policy failures retain the prior English diagnostic text.

`ProjectFile` giữ nguyên lỗi đọc file dùng chung. Lỗi parse TOML và version giữ source chain qua `Anyhow`; lỗi policy giữ nguyên nội dung tiếng Anh trước đây.
