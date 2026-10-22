# Cache entry publication / Publish entry cache

This owner creates collision-resistant temporary filenames, removes abandoned staging files through `TempPathGuard`, compares concurrent winners, and publishes immutable tarballs with a hard link. Mutable metadata publication remains last-writer-wins in `PackageCache` because it uses rename.

Owner này tạo tên file tạm tránh trùng, dọn staging còn sót bằng `TempPathGuard`, so sánh nội dung khi có writer đồng thời và publish tarball bất biến bằng hard link. Metadata có thể đổi vẫn dùng last-writer-wins bằng rename trong `PackageCache`.
