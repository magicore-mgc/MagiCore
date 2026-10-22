# Optimizer hardware detection

## Purpose

Stable `commands::optimizer::detect` facade. Shared CPU/RAM/OS facts and result types are exposed here; implementation is split into profiles, memory, GPU, and OS owners. GPU probing currently uses Linux PCI sysfs only.

## Phạm vi

Facade ổn định `commands::optimizer::detect`. Thông tin CPU/RAM/OS và type kết quả dùng chung được export tại đây; implementation tách theo profile, memory, GPU và owner hệ điều hành. Hiện chỉ Linux có probe GPU qua PCI sysfs.
