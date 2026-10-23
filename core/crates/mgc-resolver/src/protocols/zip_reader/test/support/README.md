# ZIP test-support owner

## English

Contains the in-memory ZIP builder shared by owner-local tests. This module is compiled only under `cfg(test)` and never enters production binaries. Header assembly and fixture CRC remain compatible with the reader.

## Tiếng Việt

Chứa builder ZIP trong bộ nhớ dùng chung bởi test từng owner. Module chỉ được biên dịch dưới `cfg(test)`, không đi vào binary production. Cách ghép header và CRC fixture giữ tương thích với reader.
