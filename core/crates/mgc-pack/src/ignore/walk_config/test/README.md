# Ignore-source precedence tests

These cases run as modules of the existing `test_ignore` integration target and call the stable public selection API. They verify that `.npmignore` replaces `.gitignore` and that `.gitignore` is used when `.npmignore` is absent.

READMEVN: Các case chạy trong integration target `test_ignore` qua API chọn file công khai; xác nhận `.npmignore` thay `.gitignore` và fallback `.gitignore` khi thiếu `.npmignore`.
