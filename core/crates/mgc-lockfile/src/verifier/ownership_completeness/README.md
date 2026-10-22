# Ownership completeness / Độ đầy đủ nguồn package

Owns the independent v3 gate requiring every classified package to carry one complete source identity: a non-blank registry, `registry-import` provenance with a non-blank `imported_from` lockfile name, or `delegated-tool` provenance with a non-blank tool identifier. `other` remains exempt; the gate stays separate from signature verification. Local typed failures map to `IncompleteProvenance`.

The S293 audit found no production callsite. S294 invokes this policy at the shared `serialize_lockfile` boundary used by Web, App, Lib, `dedupe` and CLI import. The importer records the actual source lockfile name instead of inventing a registry URL. The changes still require two source-review loops and runtime verification. V4 source references use a separate document model and are not validated by this function.

Owner này giữ cổng v3 độc lập: mỗi package đã phân loại phải có một source identity đầy đủ: registry khác rỗng, provenance `registry-import` kèm tên lockfile trong `imported_from` khác rỗng, hoặc `delegated-tool` kèm tên tool khác rỗng. `other` vẫn được miễn; cổng không bị gộp vào verify chữ ký. Lỗi typed cục bộ ánh xạ về category `IncompleteProvenance`.

Audit S293 chưa tìm thấy caller production. S294 gọi policy v3 tại boundary `serialize_lockfile` dùng chung của Web, App, Lib, `dedupe` và import CLI. Các thay đổi cần qua hai vòng rà source và kiểm chứng runtime. Source reference của v4 thuộc model document riêng và hàm này không xác minh chúng.
