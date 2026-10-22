# npm version metadata / Metadata version npm

Owns `PackageVersion` plus the records embedded by its npm wire contract: distribution integrity/signatures, publish configuration, author, repository, and bug-report metadata. The camelCase convention, `_id`/`_rev` names, optional fields, default values, and omitted `scripts` behavior are preserved from the frozen model.

`deprecated` delegates to the sibling `optional_deprecation` owner. That helper currently discards non-empty reason text; this batch keeps the behavior unchanged and records it for follow-up.

Quản lý `PackageVersion` cùng các record được lồng trong hợp đồng wire npm: integrity/chữ ký bản phân phối, cấu hình publish, tác giả, repository và thông tin báo lỗi. Quy tắc camelCase, tên `_id`/`_rev`, field tùy chọn, giá trị mặc định và hành vi bỏ `scripts` khi thiếu được giữ theo source đã đóng băng.

`deprecated` chuyển việc deserialize sang owner `optional_deprecation` bên cạnh. Helper hiện làm mất lý do không rỗng; batch này giữ nguyên hành vi và ghi nhận để xử lý riêng.
