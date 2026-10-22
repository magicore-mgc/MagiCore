# Transpile and cache orchestration / Điều phối transpile và cache

Owns the request-level source hash, loader selection, canonical key lookup, cache-hit path, compiler miss path, import rewrite and raw-output persistence order.

Sở hữu luồng request: hash source, chọn loader, tra key chuẩn hóa, nhánh cache hit, nhánh compiler miss, rewrite import và thứ tự lưu output raw.

A cache hit is rewritten only for its response. A miss stores raw compiler output after constructing the rewritten response, matching the existing cross-project cache contract.

Cache hit chỉ được rewrite khi tạo response. Cache miss lưu output compiler raw sau khi tạo response đã rewrite, giữ contract cache dùng chung giữa project.
