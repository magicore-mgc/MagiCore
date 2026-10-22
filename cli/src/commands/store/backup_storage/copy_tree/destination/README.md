# Destination paths

Owns destination-root creation, canonical prospective paths, parent preparation, and child-directory containment. The final destination component is checked without following; existing platform path aliases in ancestors are canonicalized before missing descendants are created. Concurrent path replacement remains an OS-level limitation of these path APIs.

# Path đích

Sở hữu tạo root đích, path dự kiến canonical, chuẩn bị parent và containment của directory con. Component đích cuối được kiểm tra không follow; alias path hợp lệ ở ancestor được canonicalize trước khi tạo descendant còn thiếu. Race thay path đồng thời vẫn là giới hạn của API filesystem dạng path.
