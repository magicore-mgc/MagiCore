# Windows process scan retry

Owns the Windows command-line retry count and the 1 ms child-exit/deadline polling interval. The 25 ms retry delay stays in the shared runner because Job Object snapshots consume it too. Refresh cadence, stop ordering, diagnostics and `WindowsProcessScan` outcomes remain unchanged.

Sở hữu số lần retry metadata command line Windows và khoảng poll 1 ms để kiểm tra child-exit/deadline. Độ trễ retry 25 ms vẫn ở runner dùng chung vì Job Object snapshot cũng sử dụng. Giữ nguyên nhịp refresh, thứ tự dừng, diagnostic và kết quả `WindowsProcessScan`.
