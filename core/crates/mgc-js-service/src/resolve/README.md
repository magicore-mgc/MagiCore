# Shared JavaScript resolution helpers / Helper resolve JavaScript dùng chung

This module preserves the public `mgc_js_service::resolve::*` API while delegating five independent responsibilities to nearest owners. `resolve.rs` owns the stable re-export surface; behavior stays in the child implementation modules.

Module này giữ API public `mgc_js_service::resolve::*` và giao năm trách nhiệm độc lập cho owner gần nhất. `resolve.rs` sở hữu bề mặt re-export ổn định; behavior nằm trong implementation của module con.

The module is used by Web's resolver compatibility facades; App shares the same service crate through manifest and registry lanes. This folder does not own registry requests, cache IO/TTL, Web lockfiles, or resolver orchestration. No local error/type module is needed by these helpers.

Resolver compatibility facade của Web dùng module này; App cùng dùng service crate qua manifest và registry. Folder không sở hữu request registry, cache IO/TTL, lockfile Web hoặc orchestration resolver. Các helper hiện không cần error/type module riêng.

Each behavior owner has a local test source and bilingual README. Existing Web resolver, cache and lockfile tests remain at their registered locations. Authored owner tests are `NOT_RUN` until the repository source-first phase closes and storage limits permit execution.

Mỗi owner hành vi có test source và README song ngữ riêng. Web resolver/cache/lockfile tests cũ vẫn ở path đã đăng ký. Test owner mới là `NOT_RUN` cho tới khi pha refactor source toàn repo khép lại và giới hạn dung lượng cho phép chạy.

The root test module checks that the former public `resolve::*` surface still exposes the forwarded helpers.

Test root kiểm tra bề mặt `resolve::*` cũ vẫn nhìn thấy các helper được chuyển tiếp.
