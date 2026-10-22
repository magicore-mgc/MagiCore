# Central-directory entry owner

## English

Parses one central-directory record, validates its complete extent against both archive and declared central-directory bounds before slicing, skips directories, rejects ZIP64 and oversized entries, reads payloads through the local-entry owner, and verifies CRC. Valid-record order and existing diagnostics remain unchanged; malformed extents fail closed. CRC and hostile-name regressions stay in the nearest test folder.

## Tiếng Việt

Đọc một central-directory record, kiểm toàn bộ kích thước trong giới hạn archive và central-directory trước khi cắt slice, bỏ qua thư mục, từ chối ZIP64 và entry quá lớn, đọc payload qua owner local-entry rồi xác minh CRC. Giữ nguyên hành vi record hợp lệ và diagnostic cũ; record lỗi bị từ chối fail-closed. Regression CRC sai và tên entry vượt biên đặt trong folder test gần nhất.
