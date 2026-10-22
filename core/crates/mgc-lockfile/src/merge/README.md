# merge

Owns the stable 3-way lockfile merge API. `mod.rs` only wires and re-exports the same public items. The `merge` implementation performs package-set reconciliation; sibling owners handle package identity indexing, the existing conflict type, and Git conflict-marker parsing. The module preserves the v2 merge semantics and diagnostic text.

# merge — trách nhiệm

Sở hữu API merge lockfile 3 chiều ổn định. `mod.rs` chỉ nối module và export lại các mục công khai hiện tại. Implementation `merge` xử lý hòa giải tập package; owner bên cạnh xử lý index identity package, conflict type hiện hữu và parse Git conflict marker. Module giữ ngữ nghĩa merge v2 cùng nguyên văn diagnostic.
