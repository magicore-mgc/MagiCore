# Compiled module cache / Cache module đã biên dịch

Owns the best-effort compiled-cache read, self-authenticating cache lookup/quarantine diagnostics, and raw JavaScript replacement write.

Sở hữu đọc compiled cache theo best-effort, tra entry tự xác thực/chẩn đoán quarantine và ghi thay thế raw JavaScript.

The caller preserves the historical sequence: open/cache-key lookup before compiling, rewrite only for the response, and persist raw output so project-specific URLs are never cached.

Caller giữ nguyên thứ tự: mở/tra cache key trước compile, chỉ rewrite response và lưu output raw để URL theo project không bị cache.
