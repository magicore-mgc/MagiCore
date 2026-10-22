# Project tsconfig state / Trạng thái tsconfig project

Owns absent/present content states and fail-closed filesystem reads for tsconfig.json. Its typed errors preserve the established English response text and retain the underlying I/O source.

Sở hữu trạng thái vắng/có nội dung và thao tác đọc filesystem fail-closed cho tsconfig.json. Error có kiểu giữ nguyên thông điệp tiếng Anh hiện hành và source I/O.

Absent and present values remain separate cache namespaces; unreadable configuration never reaches the compiler.

Giá trị vắng và có nội dung vẫn là hai namespace cache riêng; cấu hình không đọc được không bao giờ đi tới compiler.
