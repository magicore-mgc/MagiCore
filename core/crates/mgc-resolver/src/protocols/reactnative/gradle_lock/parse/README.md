# Gradle lock parsing / Parse Gradle lock

This owner skips blank/comment rows and parses pinned coordinate fields. A non-comment row with fewer than three colon-separated fields returns a typed failure instead of disappearing from the graph.

Owner này bỏ dòng trống/comment và parse các trường tọa độ đã ghim. Dòng dữ liệu có dưới ba trường phân cách bằng dấu hai chấm trả lỗi có kiểu, không biến mất âm thầm khỏi graph.

Focused cases live in `test/parse.rs`; the parent Gradle owner also retains the original public compatibility assertions.

Ca kiểm tra tập trung nằm ở `test/parse.rs`; owner Gradle cha cũng giữ assertion tương thích công khai ban đầu.
