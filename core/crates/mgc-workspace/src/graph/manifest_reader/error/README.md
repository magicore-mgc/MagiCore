# Manifest reader errors / Lỗi bộ đọc manifest

Owns only errors that escape manifest parsing. I/O and JSON parse failures retain their source and original display text. Cargo and Python TOML parse failures remain ignored by their existing readers and are not converted into errors here.
