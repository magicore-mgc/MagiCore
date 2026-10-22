# Registry-chain module / Module registry-chain

This folder is the stable owner behind the existing public chain module. Its facade exports the legacy, checked, and credential-scoped APIs without changing their paths.

Thư mục này sở hữu module chain công khai hiện có. Facade xuất API legacy, checked và API giữ phạm vi credential mà không đổi đường dẫn import.

The implementation is split into legacy loading, checked resolution, scoped values, and candidate construction. No authentication policy or error conversion is introduced here.

Implementation được tách theo legacy loading, checked resolution, giá trị có scope và xây danh sách candidate. Owner này không thêm policy xác thực hoặc conversion lỗi.
