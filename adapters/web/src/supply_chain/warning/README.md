# Supply-chain override warning

## Responsibility / Trách nhiệm
Emits the existing one-time English warning when the explicit untrusted override bypasses supply-chain guards.

In cảnh báo tiếng Anh một lần hiện có khi override untrusted tường minh bỏ qua các guard chuỗi cung ứng.

## Tests / Kiểm thử
The nearest test injects an isolated OnceLock so it does not share process-global warning state. NOT_RUN.
