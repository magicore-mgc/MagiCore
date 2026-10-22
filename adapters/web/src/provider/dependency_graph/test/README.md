# Parses `npm:` alias specs and turns dependencies, optional dependencies and non-optional peer dependencies into `ResolvedDep` edges. Optional peers remain compatibility hints and are not auto-fetched. tests / Test Parse alias `npm:` và chuyển dependencies, optional dependencies, peer không optional thành edge `ResolvedDep`. Optional peer vẫn chỉ là gợi ý tương thích, không tự tải.

Focused cases: Scoped npm alias parsing and malformed-spec rejection.

Case tập trung: Parse npm alias có scope và loại spec sai định dạng.

`mod.rs` registers `dependency_graph.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `dependency_graph.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
