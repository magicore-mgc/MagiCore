# Tarball pack coordinator / Coordinator đóng gói tarball

Owns the stable pack function and operation order: canonical roots, file selection/output exclusion, staged archive creation, digest, then final publication.

Quản lý hàm pack ổn định và thứ tự xử lý: canonical root, chọn file/loại output, tạo archive staging, hash, rồi mới publish output cuối.

The public test_tarball target remains the API and lifecycle contract for this coordinator.

Cargo target test_tarball tiếp tục là contract API và lifecycle cho coordinator này.

The nearest-owner test checks that a destination inside the project is excluded from its own archive.

Test gần owner kiểm tra output nằm trong project không bị đưa ngược vào archive.
