# Native engine build / Build native engine

Finds the supported engine crate locations and builds the artifact with locked, offline Cargo plus optimizer settings. Verifies the expected binary exists before reporting success.

Tìm các vị trí engine crate được hỗ trợ và build artifact bằng Cargo locked/offline cùng optimizer setting. Xác nhận binary đích tồn tại trước khi báo thành công.

`NativeEngineError` owns the missing-artifact diagnostic; Cargo and optimizer errors propagate. Tests remain in the CLI test owner until the later test phase.

`NativeEngineError` sở hữu diagnostic thiếu artifact; lỗi Cargo và optimizer được propagate. Test giữ tại owner CLI tới pha test sau.
