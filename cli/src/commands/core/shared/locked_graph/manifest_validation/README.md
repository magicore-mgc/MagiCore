# Locked graph manifest validation

Checks direct version ranges and the owner-scoped dependency/peer closure. The closure coordinator delegates to documented package-index, root-selection and reachable-closure owners. Ambiguous roots, foreign-owner leakage, dangling edges and orphan pins remain fail-closed.

Kiểm tra range version trực tiếp và closure dependency/peer theo owner. Root mơ hồ, package core khác, cạnh treo và pin mồ côi tiếp tục bị từ chối.
Coordinator closure chuyển tiếp tới các owner package-index, chọn root và closure reachable có README riêng.
