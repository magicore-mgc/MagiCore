# Reflink owner

## Purpose

This owner keeps the stable `mgc_platform::reflink` API and delegates to exactly one platform implementation. `mod.rs` wires and re-exports; `reflink.rs` owns the public dispatch.

## Layout

- `error/` owns error variants, Display text, and the original I/O error chain.
- `macos_clone/` owns APFS `clonefile(2)`.
- `unix_clone/` owns the existing Unix `FICLONE` path.
- `unsupported/` owns the explicit non-Unix unsupported result.
- `test/` tests public dispatch at this owner boundary.

The four existing integration tests remain in `core/crates/mgc-platform/tests/reflink.rs`.

## Chức năng

Owner này giữ API ổn định `mgc_platform::reflink` và chuyển lời gọi tới đúng một implementation theo nền tảng. `mod.rs` chỉ nối/export; `reflink.rs` điều phối API public.

## Cấu trúc

- `error/` sở hữu variant lỗi, nội dung Display và chuỗi lỗi I/O gốc.
- `macos_clone/` sở hữu `clonefile(2)` của APFS.
- `unix_clone/` sở hữu luồng `FICLONE` Unix hiện tại.
- `unsupported/` sở hữu kết quả không hỗ trợ trên non-Unix.
- `test/` kiểm tra dispatch public tại ranh giới owner này.

Bốn integration test hiện hữu vẫn nằm tại `core/crates/mgc-platform/tests/reflink.rs`.
