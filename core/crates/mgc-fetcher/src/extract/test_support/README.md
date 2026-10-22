# Extraction test support / Hỗ trợ test giải nén

## Responsibility / Trách nhiệm

Contains cfg(test)-only in-memory gzip-tar fixture builders shared by nearest extraction unit tests. Production code cannot access this module.

## Tiếng Việt

Chứa helper tạo fixture gzip-tar trong bộ nhớ, chỉ biên dịch khi test và dùng chung giữa các unit test giải nén gần source. Code production không truy cập module này.
