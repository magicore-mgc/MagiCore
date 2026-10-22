# install_execution

Coordinates four owners: the local `InstallExecution` result type, the standalone writer-gateway entry, the locked adapter/UI tail, and install-plan preparation. Stable shared re-exports stay at this owner boundary.

Điều phối bốn owner: type kết quả `InstallExecution`, entry gateway writer-lock độc lập, tail adapter/UI đã khóa và bước chuẩn bị install plan. Shared re-export ổn định tiếp tục nằm tại ranh giới owner này.

Errors keep the existing `anyhow::Result` and shared diagnostic path; this owner split does not change retry, audit or recovery policy.

Lỗi tiếp tục dùng `anyhow::Result` và diagnostic path chung; việc tách owner không đổi chính sách retry, audit hay recovery.
