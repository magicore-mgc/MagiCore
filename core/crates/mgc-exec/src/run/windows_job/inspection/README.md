# Windows Job inspection owner

This owner coordinates native inspection of members in MagiCore's private Windows Job. The effective root `ExecutionScope` reaches both active scans and root-exit inspection so the same `adb`/`xcrun` DeviceControl rule applies to Job descendants. The module facade exports existing sibling-facing operations. The coordinator contains shared private data carriers. Child owners capture confirmed Job members, inspect native process state, classify policy evidence, and convert results for the shared process-scan pipeline. The npm probe remains its own authentication owner; the error module keeps typed boundary failures.

# Owner kiểm tra Windows Job

Owner này điều phối kiểm tra native member trong Windows Job riêng của MagiCore. `ExecutionScope` hiệu lực của root đi vào cả scan đang chạy lẫn kiểm tra sau root-exit, để rule DeviceControl `adb`/`xcrun` áp dụng như nhau cho descendant thuộc Job. Facade module export thao tác sibling hiện có. Coordinator giữ carrier nội bộ dùng chung. Owner con chụp member đã xác nhận, đọc trạng thái process native, phân loại bằng chứng policy và chuyển kết quả vào pipeline process-scan. npm probe vẫn là owner xác thực riêng; module error giữ lỗi có kiểu tại ranh giới.
