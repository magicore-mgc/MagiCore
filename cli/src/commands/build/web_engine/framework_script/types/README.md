# Framework launch types / Kiểu launch framework

`BuildLaunch` is the shared internal value returned by validated command mapping: executable path, argument vector and extra environment pairs. It is data only; it owns no policy, error domain or side effect.

`BuildLaunch` là giá trị nội bộ dùng chung do command mapping đã kiểm tra trả về: executable path, vector argument và cặp environment bổ sung. Đây chỉ là data, không sở hữu policy, error domain hay side effect.

The alias remains crate-local so command callers and tests can preserve inferred use without creating a public API commitment.

Alias giới hạn trong crate để caller và test tiếp tục dùng kiểu suy luận mà không tạo cam kết public API.
