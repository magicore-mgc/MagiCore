# Direct dependency downgrade guard

## Responsibility / Trách nhiệm
Loads a valid manifest-satisfying prior Web lock, derives previous direct pins, selects the maximum current resolution matching each direct range, and rejects regressions.

Đọc lock Web cũ còn thỏa manifest, lấy pin trực tiếp trước đó, chọn resolution hiện tại lớn nhất khớp range và từ chối regression.

## Boundary / Ranh giới
Nested/transitive versions are intentionally excluded from the baseline. Invalid or absent lock state retains the existing no-baseline pass behavior; source-level risks are recorded in S530.

Version nested/transitive cố ý không tham gia baseline. Lock sai hoặc vắng mặt giữ semantics hiện tại là không có baseline để so; rủi ro source được ghi trong S530.
