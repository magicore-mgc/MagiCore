# Go manifest owners / Owner manifest Go

Keep Go module parsing of replace/exclude directives separate from writing the MGC-owned require set. Tách parse replace/exclude khỏi việc ghi require set do MGC quản lý.

Tests remain at adapters/lib/src/test/manifest_test.rs until the deferred test phase. Test giữ tại adapters/lib/src/test/manifest_test.rs đến phase test sau.
