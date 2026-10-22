# Attestation records / Record chứng thực

Defines the serialized `PreviousKey`, `Attestation` and `AttestationStatus` types. Field names, serde defaults and unknown-field rejection stay part of the existing on-disk format.

Định nghĩa các type serialize `PreviousKey`, `Attestation` và `AttestationStatus`. Tên field, serde default và từ chối field lạ là một phần của định dạng lưu trữ hiện hành.

The owner keeps the public field contract and uses serde directly; it adds no error abstraction. The nearest test checks defaulted previous-key history and strict unknown-field rejection.

Owner giữ nguyên public field và dùng serde trực tiếp; không thêm lớp error mới. Test gần nhất kiểm tra lịch sử key mặc định và từ chối field không biết.
