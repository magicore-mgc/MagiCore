# Cargo root resolution owner

EN: Maps the manifest's direct dependency names to the locked package graph and sorts Cargo labels.

VI: Ánh xạ tên dependency trực tiếp sang package graph đã lock và sắp xếp nhãn Cargo.

Dependency option validation is delegated to the sibling `dependency_specs/` owner after root equality checks, preserving the former validation order.

Việc kiểm tra option dependency được giao cho owner `dependency_specs/` sau khi đối chiếu root, giữ nguyên thứ tự kiểm tra trước đây.
