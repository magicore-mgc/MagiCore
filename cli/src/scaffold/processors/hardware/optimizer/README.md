# Hardware optimizer marker

## Responsibility
Writes optimizer.json for a package that will be consumed by the optimizer command.

## Phạm vi
Ghi optimizer.json cho package được lệnh optimizer sử dụng.

## Boundary
This writer creates marker/config data only. It does not spawn hardware toolchains or benchmark processes.

## Ranh giới
Writer này chỉ tạo dữ liệu marker/config. Nó không spawn toolchain phần cứng hoặc process benchmark.
