# .NET manifest owners / Owner manifest .NET

Read and mutate PackageReference entries only when the flat manifest can preserve their meaning; unsupported semantics fail closed. Chỉ đọc/mutate PackageReference khi manifest phẳng giữ được ý nghĩa; semantics chưa hỗ trợ phải fail-closed.

Tests remain at adapters/lib/src/test/manifest_test.rs until the deferred test phase. Test giữ tại adapters/lib/src/test/manifest_test.rs đến phase test sau.
