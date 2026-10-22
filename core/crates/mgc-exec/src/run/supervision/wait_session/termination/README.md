# Termination

Owns the platform-aware process-tree termination boundary and direct-child kill fallback for a wait session.

Unix uses the existing tracked process-tree terminator; Windows terminates through the existing Job guard; unsupported platforms preserve the existing no-op tree termination result. Error wrapping retains the original trigger plus termination/child-kill causes.

## Test ownership

Failure source-chain and platform termination behavior belong in this owner’s tests in the later verification pass.

# Dừng cây process

> Owner này quản lý ranh giới dừng cây process theo nền tảng và fallback kill child trực tiếp.

> Unix vẫn dùng tracked terminator hiện có; Windows vẫn terminate qua Job guard; nền tảng khác giữ kết quả no-op như trước. Lỗi bọc vẫn giữ trigger gốc và nguyên nhân terminate/kill.
