# Request execution owner

EN: Owns request cloning, authentication insertion, the single pre-loop rate-limit wait, retry status/error classification, backoff, and response/error return. Every verb passed through execute, including POST and PATCH, can be replayed for the current retry conditions; this may duplicate side effects. The current counter permits the initial send plus three retries. This batch preserves both behaviors for a separate policy fix.

VI: Sở hữu clone request, gắn xác thực, chờ rate-limit một lần trước vòng lặp, phân loại status/lỗi retry, backoff và trả response/lỗi. Mọi verb đi qua execute, kể cả POST/PATCH, có thể bị gửi lại theo điều kiện retry hiện tại và tạo side effect trùng. Bộ đếm cho phép lần gửi đầu cộng ba lần retry. Batch này giữ nguyên cả hai để xử lý policy riêng sau.
