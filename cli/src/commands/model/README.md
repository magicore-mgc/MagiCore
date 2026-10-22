# Model command / Lệnh model

English: This folder owns the `mgc model` CLI contract and routes each operation to its focused implementation owner.
Tiếng Việt: Thư mục này sở hữu hợp đồng lệnh `mgc model` và chuyển từng thao tác đến owner triển khai tương ứng.

## Operation owners / Owner thao tác

- `types/`: clap arguments, variants, defaults and environment-backed token option.
- Operation owners keep typed failures in their local `error.rs` when the failure belongs to that owner; the shared `parse_manifest_failed` message remains in the common diagnostic owner.
- `dispatch/`: route `push`, `pull`, `list`, `rm` and `quantize` without reimplementing them.
- `manifest_types/`, `manifest_path/`, `manifest_io/`: persisted shape, safe path rules and atomic local manifest I/O.
- `cas_pull/`, `huggingface/`, `oci_pull/`: download source routing and import into the CAS.
- `local_models/`: local inventory, safe removal and blob reference checks.
- `registry_client/`, `registry_push/`, `registry_pull/`, `registry_catalog/`: OCI registry client and operations.
- `quantize/`: unsupported native quantization remains fail-closed.

The facade in `mod.rs` preserves `commands::model::{ModelArgs, ModelCmd, run}` and the existing test registration. Model-only failures now use the operation-local typed errors; compatibility helpers remain under `cli/src/error/messages/model/`, and `parse_manifest_failed` remains shared with npm publish.

Facade trong `mod.rs` giữ nguyên đường dẫn API công khai và đăng ký test hiện tại. Giai đoạn này giữ nguyên các chẩn đoán đang được owner lỗi model cung cấp; chưa đổi hợp đồng lỗi dùng chung với lệnh publish.
