# Lane runner / Bộ chạy lane

`runner.py` coordinates phases; `preflight.py` probes declared toolchains; `context.py` owns one sandbox and step environment; scaffold, preparation, ordered steps, run/dev probes, install evidence, optimizer and recovery are separate owners. Process-group termination, port sweep, timeouts and prerequisite blocking stay explicit.

`runner.py` điều phối phase; `preflight.py` probe toolchain; `context.py` giữ sandbox và môi trường; scaffold, preparation, lifecycle step, run/dev probe, install evidence, optimizer và recovery có owner riêng. Kill process group, quét port, timeout và block prerequisite vẫn tường minh.
