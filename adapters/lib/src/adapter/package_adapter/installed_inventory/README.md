# installed_inventory

## Responsibility

Return installed-artifact inventory only for lanes that can verify it: the embedded Web adapter and native Python installation. Other lanes return the existing explicit unsupported diagnostic.

## Ghi chú tiếng Việt

Folder này trả inventory artifact đã cài chỉ ở lane có thể xác minh: Web adapter nhúng và Python native. Lane khác tiếp tục trả lỗi unsupported rõ ràng; khai báo trong manifest hoặc lock không được coi là bằng chứng package đã cài.
