# File import mode / Chế độ import file

This owner validates the source path, selects streaming or buffered import from the configured store threshold, and delegates verified input to the existing CAS writer. Streamed imports own a create-new staging path with drop cleanup until atomic publication; failures after partial writes cannot leave a reusable orphan. The nearest behavior tests cover this staging lifecycle.

Owner này xác minh source path, chọn stream hoặc buffer theo threshold của store và chuyển input hợp lệ sang CAS writer. Import dạng stream sở hữu staging tạo mới và tự dọn khi drop cho đến lúc publish atomic; lỗi sau khi ghi dở không để lại file mồ côi có thể bị tái sử dụng. Test lifecycle đặt sát owner này.
