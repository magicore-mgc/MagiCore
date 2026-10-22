# AI model-install lifecycle

This owner coordinates the existing download and optional checksum-verification steps, then returns the stable `InstallSummary`. It does not own transport, checksum algorithms, source validation, or storage policy.

Owner này điều phối các bước tải hiện có và xác minh checksum khi nguồn khai báo, sau đó trả `InstallSummary` ổn định. Owner không nắm transport, thuật toán checksum, kiểm tra nguồn hay chính sách lưu trữ.

The two lifecycle tests are wired from this owner and remain unchanged. Tests are deferred until repository-wide source closure.

Hai test vòng đời được đăng ký từ owner này và giữ nguyên. Test sẽ chạy sau khi đóng refactor source toàn repository.
