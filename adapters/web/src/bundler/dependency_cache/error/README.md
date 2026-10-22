# Dependency bundle errors

This private error owner retains esbuild diagnostics and post-build metafile-gate failures until the cache boundary maps them to the existing `None` result and warning. New gate diagnostics omit filesystem paths.

Owner lỗi private này giữ diagnostic esbuild và lỗi gate metafile sau build tới boundary cache, nơi chúng được map về `None` và warning hiện tại. Diagnostic gate mới không chứa filesystem path.
