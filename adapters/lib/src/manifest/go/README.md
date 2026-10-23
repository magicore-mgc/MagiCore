# Go manifest owners / Owner manifest Go

Keep Go module parsing of replace/exclude directives separate from writing the MGC-owned require set. Tách parse replace/exclude khỏi việc ghi require set do MGC quản lý.

Tests remain at adapters/lib/tests/integration/manifest_io/ until the deferred test phase. Test giữ tại adapters/lib/tests/integration/manifest_io/ đến phase test sau.
