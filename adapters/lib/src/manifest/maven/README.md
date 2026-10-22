# Maven manifest owners / Owner manifest Maven

Parse and write pom.xml while preserving unmanaged blocks, fail-closed rules, exact-version checks, and atomic publication. Đọc/ghi pom.xml, giữ block không thuộc MGC, quy tắc fail-closed, kiểm tra version chính xác và publish nguyên tử.

Each responsibility has a named owner folder. Tests remain at adapters/lib/src/test/manifest_test.rs until the deferred test phase. Mỗi trách nhiệm có folder owner cùng tên. Test giữ tại adapters/lib/src/test/manifest_test.rs đến phase test sau.
