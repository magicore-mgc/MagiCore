# AI core command owners / Owner lệnh AI core

`mod.rs` preserves the feature-gated `commands::core::ai::*` routes and the crate-visible optimizer-runtime helper. Responsibilities are split into project identity, native-Python lifecycle gate, AI dev runner, optimizer runtime detection, and local typed errors. The `core::shared::*` facade and add/install/update/remove/list/dev handlers remain their current callers. Framework/runtime types stay in their adapter/runtime owners; no type is duplicated. The packaged `mgc dev --core ai` journey remains covered by `cli/tests/native_python_runtime_e2e.rs`.

Four existing local tests were moved without changing their assertions; four focused owner tests were added. All eight are authored but NOT_RUN. No support claim or runtime behavior changes in this structural batch.

# Owner lệnh AI core

`mod.rs` giữ route feature-gated `commands::core::ai::*` và helper optimizer-runtime crate-visible. Trách nhiệm được tách thành nhận diện project, gate lifecycle Python native, runner dev AI, nhận diện optimizer runtime và lỗi typed cục bộ. Facade `core::shared::*` cùng handler add/install/update/remove/list/dev vẫn là caller hiện tại. Type framework/runtime vẫn ở adapter/runtime owner, không nhân bản. Hành trình binary đóng gói `mgc dev --core ai` vẫn được test tại `cli/tests/native_python_runtime_e2e.rs`.

Bốn test local cũ được chuyển nguyên assertion; thêm bốn test owner tập trung. Cả tám đã viết nhưng NOT_RUN. Batch cấu trúc này không đổi claim support hay hành vi runtime.
