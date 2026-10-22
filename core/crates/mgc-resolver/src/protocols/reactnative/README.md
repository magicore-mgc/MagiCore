# React Native resolver / Resolver React Native

This owner preserves the public `protocols::reactnative` surface while keeping the Gradle lock parser, Podfile.lock parser, CocoaPods client, URL rule, dependency extraction, and tier markers in separate owners. The JS tier is still delegated to the Web adapter; Android pins are exact Maven coordinates; iOS specs are fetched and checked against the Podfile.lock SHA-1.

Folder này giữ nguyên API `protocols::reactnative`, đồng thời tách parser Gradle, parser Podfile.lock, client CocoaPods, quy tắc URL, trích dependency và marker tier thành owner riêng. Tier JS vẫn ủy quyền adapter Web; pin Android là tọa độ Maven chính xác; spec iOS được tải và so SHA-1 với Podfile.lock.

`mod.rs` only wires modules and re-exports. `reactnative.rs` owns stable lock-marker constants. The app resolver caller is intentionally unchanged. Test files are attached to the nearest behavior owner; none were executed during the source-first pass.

`mod.rs` chỉ nối module và export. `reactnative.rs` sở hữu hằng marker lock ổn định. Caller resolver của app được giữ nguyên. Test đặt sát owner hành vi; chưa chạy test trong lượt refactor source-first.

This module does not make React Native multi-tier installation a supported lifecycle. Existing app integration cases in `adapters/app/tests/native_rn_tests.rs` exercise the layered resolver and assert that install returns `Unsupported` until atomic cross-tier install exists.

Module này chưa biến cài đặt đa tier React Native thành lifecycle được hỗ trợ. Các integration case hiện có trong `adapters/app/tests/native_rn_tests.rs` gọi resolver phân tầng và xác nhận install trả `Unsupported` cho tới khi có cài đặt cross-tier nguyên tử.
