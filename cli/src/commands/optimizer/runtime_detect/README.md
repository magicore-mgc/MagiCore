# Optimizer runtime detection facade

## Purpose

This folder preserves `commands::optimizer::runtime_detect` as the stable caller-facing module. It exports the existing runtime types and detection functions while implementation lives in named responsibility owners. Core-specific evidence remains separated: Web, Cloud, AI, Lib, and App. Shared Node package-manager selection, Python manifests, and safe project-file reads each have one owner. Game, IoT, CI/CD, hardware, and other generic core IDs keep the current Cargo-manifest Rust fallback in dispatch.

## Phạm vi

Thư mục này giữ nguyên đường dẫn module `commands::optimizer::runtime_detect` cho caller. Nó export lại type và hàm hiện có; phần xử lý nằm trong owner có tên. Web, Cloud, AI, Lib và App có detector riêng. Chọn package manager Node, đọc manifest Python và đọc file project an toàn là các owner dùng chung. Game, IoT, CI/CD, hardware và core chung tiếp tục dùng fallback Rust theo Cargo manifest ở tầng điều phối.

Các detector hiện tại đọc manifest theo hướng best-effort, không follow symlink ở shared reader, và trả `Unknown` khi không có bằng chứng. Không thêm error type vì API hiện hành không trả lỗi ở boundary này. Test điều phối nằm cạnh implementation trong `runtime_detect/test/`; test theo core nằm cạnh owner tương ứng. Runtime test chưa chạy trong phase source-first.
