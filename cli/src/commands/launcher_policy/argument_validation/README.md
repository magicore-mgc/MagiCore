# Argument validation / Kiểm tra arguments

Runs the existing ordered policy: always-blocked runtime flags first, then disallowed Deno permissions. Both passes keep the original argument order, profile order, `starts_with` matching, early return, and `anyhow::Result<()>` boundary. Error construction is owned by the neighboring [error module](../error/README.md).

The profile lookup now returns static slices, so checking arguments does not allocate one or more temporary vectors per argument. The change does not relax or expand the policy. Bun, Deno, and Node behavior tests live in this folder's `test/` directory.
