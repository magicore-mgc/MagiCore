# Optimizer execution

## Purpose

Owns the `optimize_project` flow: hardware profile reporting, runtime detection, fail-closed refusal when nothing can be optimized, adapter generation and guarded file writes. `mod.rs` only wires and exports `optimize/optimize.rs`; adapter predicates stay in `../adapters/`, runtime recognition in `../runtime_detect/`.

## Phạm vi

Owner này sở hữu luồng `optimize_project`: báo cáo profile phần cứng, phát hiện runtime, từ chối fail-closed khi không tối ưu được gì, sinh cấu hình qua adapter và ghi file có kiểm soát. `mod.rs` chỉ wiring và export `optimize/optimize.rs`; điều kiện adapter nằm ở `../adapters/`, nhận diện runtime ở `../runtime_detect/`.
