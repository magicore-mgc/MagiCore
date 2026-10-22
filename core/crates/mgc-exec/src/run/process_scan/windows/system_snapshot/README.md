# Windows system snapshot

Owns the Windows-only `sysinfo` refresh adapter, query-only native process creation-time probe, and FILETIME-to-Unix conversion constants used by the existing Windows integration test seam. The unsafe `OpenProcess`/`GetProcessTimes`/`CloseHandle` sequence remains unchanged and test-gated.

Sở hữu adapter refresh `sysinfo`, probe creation-time native chỉ truy vấn và các constant chuyển FILETIME sang Unix dùng bởi test seam tích hợp Windows. Chuỗi unsafe `OpenProcess`/`GetProcessTimes`/`CloseHandle` giữ nguyên và chỉ bật trong test.
