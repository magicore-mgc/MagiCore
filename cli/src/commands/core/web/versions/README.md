# Web scaffold version owners

EN: The `web::versions` facade keeps stable type and function routes while separate owners load release-age policy, filter registry metadata, resolve seed ranges/tags, make npm requests, and construct local errors.

VI: Facade `web::versions` giữ route type/function ổn định; các owner riêng đọc age policy, lọc metadata registry, resolve seed range/tag, gọi npm registry và dựng lỗi tại chỗ.

All package version choices still go through MagiCore. The age-gated route remains fail-closed, and the feature-disabled route keeps its existing non-Web behavior. Test bodies remain separate under the owner's `test/` folders.

Mọi lựa chọn version package vẫn đi qua MagiCore. Nhánh có age gate vẫn fail-closed; nhánh tắt Web giữ hành vi non-Web hiện tại. Test body được tách riêng trong `test/` của owner.
