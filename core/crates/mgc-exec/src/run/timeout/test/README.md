# Timeout owner tests

`error.rs` covers positive seconds, absent-value defaulting, zero/format/unicode rejection, source retention, and the rule that explicit or disabled timeout modes do not read the environment. The tests inject environment values and do not mutate process-global environment state.

# Test owner timeout

`error.rs` kiểm tra giây dương, mặc định khi thiếu giá trị, từ chối zero/sai định dạng/sai Unicode, giữ source và đảm bảo timeout tường minh hoặc bị tắt sẽ không đọc environment. Test inject giá trị, không sửa environment toàn process.
