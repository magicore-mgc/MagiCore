# processor

Scaffold implementation owners behind the stable `Scaffolder` facade.
Các owner triển khai scaffold phía sau facade `Scaffolder` ổn định.

Owners separate target claim/atomic publication, framework definitions, output writers, online generator copy, template lookup/layers, materialization/path safety, token contexts, manifest schema and generated common files. Existing error factories remain under `cli/src/scaffold/error.rs`; no duplicate error catalog is introduced.

Các owner tách claim/publish nguyên tử, framework, writer, copy output online, tìm/layer template, materialize/path safety, context token, schema manifest và file chung. Factory lỗi hiện tại tiếp tục ở `cli/src/scaffold/error.rs`; không tạo catalog lỗi trùng.

The processor unit tests live next to this owner under `test/`; the `tests` child module and all existing harness names are preserved by same-scope include wiring. Their sources have been moved, but remain unexecuted until the repository-wide source pass and disk gate are complete.

Unit test của processor nằm cạnh owner này trong `test/`; module con `tests` và tên harness hiện hữu được giữ bằng include wiring cùng scope. Source test đã được chuyển nhưng chưa chạy cho tới khi xong refactor toàn repo và đạt giới hạn dung lượng.

Executable verification remains deferred until the whole-repository production-source pass is complete.
Chưa xác minh bằng chương trình chạy cho đến khi hoàn thành lượt refactor production source toàn repository.
