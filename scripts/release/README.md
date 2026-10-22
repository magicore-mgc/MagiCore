# Release scripts / Script phát hành

This owner contains small validation programs used by release workflow jobs. / Owner này chứa chương trình kiểm tra nhỏ được các job phát hành gọi.

## verify_audit_capability_matrix.py

Validates docs/specs/auditCapabilityMatrix.json against its source-controlled expected_lanes scope, the current GITHUB_SHA, required evidence kinds, supported status, and explicit missing-lane records. A mismatch prints every blocking reason and exits non-zero. It reads the checked-out repository from its root and writes no files.

release-audit-capability.yml owns runner/tool setup and artifact upload; this script owns only the final fail-closed evidence decision. Add focused positive and negative tests beside this owner during the repository-wide test phase. Those tests have not been created or run in this source-first batch.

Script nhận docs/specs/auditCapabilityMatrix.json, kiểm tra phạm vi expected_lanes từ source, GITHUB_SHA hiện tại, loại evidence bắt buộc, trạng thái supported và các lane bị thiếu. Sai lệch sẽ in các lý do chặn và trả mã khác 0. Script đọc repository tại thư mục gốc và không ghi file.

release-audit-capability.yml sở hữu việc chuẩn bị runner/toolchain và upload artifact; script này chỉ quyết định evidence có đủ điều kiện phát hành hay không. Bổ sung test dương/âm gần owner trong giai đoạn kiểm thử toàn repository; batch source-first này chưa tạo hoặc chạy các test đó.

## Focused release operations

- build/: artifact-name selection and Unix/Windows SBOM generation.
- artifact_smoke/: Web install smoke against the unpacked release binary.
- prepublish/: archive-set, attestation, and SBOM validation before publication.

Each owner uses bounded environment inputs. Nearest unit/contract tests are deferred until the repository-wide test phase; none have been run for this refactor.
