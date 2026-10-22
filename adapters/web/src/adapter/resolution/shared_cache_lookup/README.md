# Shared resolution cache lookup / Tra cứu shared resolution cache

Reads a shared graph cache only for a cache-eligible resolve, when no age gate is active, and when it contains exactly one explicit, range-compatible direct root for each manifest dependency. Fresh resolves always miss this cache. A validated hit is checked against the prior owner-scoped lock root before return; ambiguous legacy cache entries miss and continue through the resolver.

Tra cứu shared resolution cache chỉ trong resolve được phép dùng cache, khi không có age gate và graph có đúng một root trực tiếp tường minh, tương thích range cho mỗi dependency manifest. Fresh resolve luôn miss cache. Cache hit hợp lệ phải qua guard so với root pin lock theo owner trước khi trả về; entry cache cũ mơ hồ bị xem là miss và đi tiếp qua resolver. Test gần source ở `test/`; mọi test vẫn `NOT_RUN`.
