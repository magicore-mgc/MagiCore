# Scoped registry value / Giá trị registry có scope

This owner defines the public value that keeps a Registry paired with its optional credential origin/path scope. Its fields and visibility are part of the existing API.

Owner này định nghĩa giá trị công khai ghép Registry với scope credential origin/path tùy chọn. Các field và visibility là một phần API hiện có.

The type has no token-specific Debug implementation and the extraction adds no secret formatting behavior. Scoped transport tests remain in their existing integration suite.

Type này không có Debug riêng xử lý token và lần tách không thêm hành vi format secret. Test scoped transport vẫn ở integration suite hiện có.
