# package_index

Builds the deterministic merge identity map keyed by core owner, ecosystem and package name. Exact duplicate records collapse; conflicting records with one identity fail through the existing `MergeConflict` contract. Nearest-owner tests cover both outcomes.

# package_index — trách nhiệm

Tạo identity map xác định theo core owner, ecosystem và tên package. Bản ghi trùng hoàn toàn được gộp; bản ghi xung đột cùng identity trả lỗi theo contract `MergeConflict` hiện có. Test gần owner khóa cả hai hành vi.
