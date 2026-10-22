# Signing-key rotation continuity / Duy trì liên tục rotation key

Selects the previous public key only when a stored attestation parses and its key id differs from the selected key. Missing, malformed and same-key slots still produce no previous key; history remains bounded to one generation.

Chỉ chọn public key cũ khi attestation đã lưu parse được và key id khác key đang chọn. Slot thiếu/lỗi hoặc cùng key tiếp tục không thêm key cũ; lịch sử giữ tối đa một đời.

Uses the existing `PreviousKey` data contract and no new error boundary. Nearest tests cover first/same/different/malformed states.

Dùng nguyên data contract `PreviousKey`, không thêm error boundary. Test gần nhất bao phủ lần đầu/cùng key/khác key/slot lỗi.
