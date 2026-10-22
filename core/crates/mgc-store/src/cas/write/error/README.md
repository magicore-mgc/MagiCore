# CAS write error mapping / Ánh xạ lỗi ghi CAS

Owns the typed local operation labels and maps filesystem failures into the shared `StoreError::Io` category without changing the operation wording or affected path.

Owner này giữ các nhãn thao tác cục bộ có kiểu và ánh xạ lỗi filesystem sang category `StoreError::Io` dùng chung, không đổi nội dung thao tác hoặc path gây lỗi.
