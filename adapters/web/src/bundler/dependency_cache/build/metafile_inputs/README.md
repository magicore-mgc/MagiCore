# Dependency metafile input gate

This owner parses esbuild's dependency-bundle metafile and rejects output when any reported input cannot be canonicalized or is outside the installed package roots. The check runs after esbuild reads the files, so it prevents rejected content from entering the session cache/HTTP response but does not sandbox filesystem reads.

Owner này đọc metafile của dependency bundle và từ chối output nếu input được báo không canonicalize được hoặc nằm ngoài root package đã cài. Check chạy sau khi esbuild đọc file: nó chặn nội dung bị từ chối vào cache/HTTP response nhưng không sandbox thao tác đọc filesystem.
