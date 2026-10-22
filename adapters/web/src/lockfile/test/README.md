# Lockfile tests / Test lockfile

EN: The previous lockfile unit suite is split by signed writes, v3 roots, graph restore, cross-core behavior, filesystem safety, reader formats and v4 schema/merge/signature subjects.
VI: Bộ unit test lockfile cũ được chia theo ghi lock ký số, root v3, phục hồi graph, đa core, an toàn filesystem, reader format và schema/merge/chữ ký v4.

Each module is registered with `#[path]` from `src/test/unit_tests/unit_tests.rs`; it remains a child of the original parent and keeps the same shared fixture/import scope. Test bodies and assertions were copied unchanged. Tests were not executed in S169.
