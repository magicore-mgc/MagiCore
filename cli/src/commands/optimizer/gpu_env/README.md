# GPU fact env file

## Purpose

Owns generating `.mgc-optimizer/gpu.env` from measured GPU facts only: every unmeasured field is emitted as a comment, never as an invented value, and control characters are sanitized so the file stays parse-only data.

## Vietnamese

Sở hữu việc sinh `.mgc-optimizer/gpu.env` chỉ từ fact GPU đo được: field chưa đo được ghi thành comment, không bịa giá trị, và ký tự điều khiển được lọc để file chỉ là dữ liệu parse.
