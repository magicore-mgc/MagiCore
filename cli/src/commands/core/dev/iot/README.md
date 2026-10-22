# IoT dev command / Lệnh dev IoT

## Responsibility / Trách nhiệm

Preserves `mgc flash` entrypoint and coordinates framework/board validation, build, artifact selection and audited flashing.

## Tests / Kiểm thử

Board resolution, ELF selection and executor policy have nearby tests; hardware flashing is not invoked by unit tests.

## Tiếng Việt

Folder này giữ entrypoint `mgc flash`, điều phối kiểm tra framework/board, build, chọn artifact và flash qua công cụ được kiểm soát. Test board/ELF/executor ở gần owner; unit test không flash phần cứng.
