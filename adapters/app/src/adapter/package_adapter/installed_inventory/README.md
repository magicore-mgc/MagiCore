# installed_inventory

## Responsibility

Return an empty inventory for an App manifest with no dependencies. When dependencies exist, keep the current explicit unsupported refusal because the adapter does not reconcile resolved locks with installed files.

## Ghi chú tiếng Việt

Folder này trả inventory rỗng khi manifest App không khai báo dependency. Nếu có dependency, giữ lỗi unsupported rõ ràng vì adapter chưa đối chiếu lock đã resolve với file thực sự được cài.
