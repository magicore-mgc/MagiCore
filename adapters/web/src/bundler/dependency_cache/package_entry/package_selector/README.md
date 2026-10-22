# Package selector

Splits plain or scoped package identifiers from optional subpaths, then resolves subpaths as files or directories under the canonical package root. Package-root symlinks remain compatible with pnpm stores and workspace links; subpath symlinks must stay within the resolved package root.

Tách tên package thường/scoped khỏi subpath tùy chọn, sau đó resolve subpath dạng file hoặc thư mục trong canonical package root. Symlink package-root vẫn tương thích pnpm store và workspace; symlink subpath phải nằm trong package root đã resolve.
