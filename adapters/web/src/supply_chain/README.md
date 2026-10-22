# Web supply-chain guards

## Purpose / Chức năng
Owns the Web resolver's publish-age quarantine and top-level downgrade guards. The public entry remains `crate::supply_chain::enforce_resolution_supply_chain_guards`; implementation is split by configuration, decision plan, warning, publish age, downgrade selection, and local policy errors.

Sở hữu guard cách ly tuổi phát hành và chống hạ version ở resolver Web. Giữ entry công khai; implementation tách theo cấu hình, kế hoạch quyết định, cảnh báo, publish age, chọn version chống downgrade và lỗi policy cục bộ.

## Contract / Hợp đồng
The coordinator runs after graph assembly and before resolution publication. Project `[security] min_release_age` values are hours and are converted to seconds; the local release-policy store is already in seconds. Invalid project/store reads fail closed. Missing or invalid registry timestamps are rejected by an active age guard unless `[security] allow_missing_time = true`; using that escape prints one generic warning. Age-policy configuration and both strict-age environment switches bypass lock/shared-cache short-circuits. The downgrade guard consumes indexed direct-root selections from `Resolver::solve_with_roots`; it never infers a root pin from same-name transitive versions. Shared-cache hits require one explicit compatible root per dependency and pass the same prior-lock downgrade guard. A missing or stale lock entry removes the baseline only for that dependency, while unreadable/malformed lock data fails closed. Root package rows must match both Web ecosystem and owner, except for the legacy Web v2 shape. `MAGICORE_ALLOW_UNTRUSTED=1` remains the explicit warned bypass for both guards.

Coordinator chạy sau khi dựng graph và trước khi publish resolution. `min_release_age` trong project tính bằng giờ, được đổi sang giây; policy trong local store vốn là giây. Lỗi đọc project/store bị từ chối. Guard tuổi đang bật sẽ chặn timestamp registry thiếu hoặc sai trừ khi cấu hình rõ `[security] allow_missing_time = true`; mỗi process phát một cảnh báo chung khi escape này được dùng. Cấu hình tuổi và hai biến strict-age sẽ bỏ qua short-circuit lock/shared-cache. Guard downgrade dùng lựa chọn root trực tiếp có index từ `Resolver::solve_with_roots`; không suy ra pin từ version transitive trùng tên. Cache hit cần root trực tiếp tường minh tương thích cho từng dependency và cũng qua guard so với lock cũ. Row package root phải đúng ecosystem Web và owner, trừ format Web v2 cũ. Lock entry thiếu/cũ chỉ bỏ baseline của dependency đó; lock không đọc/parse được thì bị từ chối. `MAGICORE_ALLOW_UNTRUSTED=1` vẫn là bypass tường minh có cảnh báo cho cả hai guard.

## Owners / Owner
- configuration/: project/database age policy, unit normalization, and boolean environment parsing.
- types/guard_plan/: pure policy decision derived from configuration.
- warning/: one-time warnings for the explicit untrusted override and missing-time escape.
- publish_age/: quarantine check across resolved metadata.
- downgrade/: compare exact indexed direct-root versions with valid prior lock pins.
- error/: typed local refusals wrapped in `MgError::Anyhow` with their source chains retained.

## Verification / Xác minh
Nearest source tests are registered under each behavioral owner. The S530/S531/S532 cases are authored but remain `NOT_RUN` pending repository source closure and the storage gate. Later run the resolver crate tests, mgc-web-adapter suite, and resolver E2E lane.

Test gần source được đăng ký tại từng owner có hành vi. Test S530/S531/S532 đã viết nhưng vẫn `NOT_RUN` đến khi đóng source toàn repo và đạt storage gate. Sau đó chạy test crate resolver, suite mgc-web-adapter và E2E resolver.
