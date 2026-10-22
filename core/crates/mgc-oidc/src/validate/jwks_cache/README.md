# JWKS cache

English: Owns one process-static issuer map, per-issuer async refresh lock, TTL check and failed-refresh throttle. Capacity is 32 issuer slots with no eviction; do not describe it as leak-free or benchmarked.

Tiếng Việt: Sở hữu map issuer tĩnh theo process, khóa refresh async từng issuer, kiểm tra TTL và throttle khi refresh lỗi. Sức chứa là 32 issuer, không eviction; chưa có benchmark hoặc bằng chứng về peak memory.

The 1 MiB HTTP response cap is enforced in the shared fetch layer before JSON decoding. This owner preserves current cache semantics; eviction/capacity policy is a separate follow-up.
