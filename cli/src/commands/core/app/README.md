# App core shared helpers / Helper dùng chung của App core

`mod.rs` preserves the existing `commands::core::app::*` routes. `project_identity/` owns project-root and language detection; `react_native_gate/` owns the React Native dependency refusal; `manifest_hint/` maps unsupported CLI lanes to their manifests; `error/` owns typed App diagnostics and the two crate-visible error builders. The native App install implementation remains under `core/install/app/`.

`docs/` tracks this structural batch. Each behavior owner has a same-name implementation, a wiring-only `mod.rs`, and nearest-owner tests. The inherited three tests were moved without changing their assertions; three focused tests were added. All six are authored and NOT_RUN. This batch does not change App support claims or runtime behavior.

# Helper dùng chung của App core

`mod.rs` giữ nguyên các route `commands::core::app::*`. `project_identity/` sở hữu nhận diện root/ngôn ngữ; `react_native_gate/` sở hữu việc từ chối thao tác dependency React Native; `manifest_hint/` ánh xạ lane chưa có CLI sang manifest; `error/` sở hữu diagnostic App có kiểu và hai hàm dựng lỗi crate-visible. Implementation cài App native vẫn nằm trong `core/install/app/`.

`docs/` theo dõi batch cấu trúc này. Mỗi behavior owner có implementation cùng tên, `mod.rs` chỉ nối module/export và test gần owner. Ba test kế thừa được chuyển mà không đổi assertion; có thêm ba test tập trung. Cả sáu mới được viết, chưa chạy (NOT_RUN). Batch này không đổi claim hỗ trợ App hay hành vi runtime.
