# shared_cache_isolation

## Responsibility

Owns the lifetime and handoff of one shared-cache root for a benchmark scenario.

## Inputs and behavior

The caller may pass a seeded root; otherwise this owner creates a temporary directory and keeps it alive until the scenario closure returns. It passes the selected path to the closure. The scenario then constructs `WebAdapter` through `with_registry_and_shared_cache_root`, which validates the registry and writable cache root and bypasses process cache discovery.

This owner does not set, remove, or restore process environment variables. It also does not change product metadata TTL settings. Each scenario creates a fresh provider/cache root unless it explicitly receives a seeded cache. Product environment configuration remains in effect.

## Errors and tests

Temporary-directory creation and scenario failures propagate through `anyhow::Result`; no fallback to a global cache is allowed. Tests for path lifetime, scenario isolation and concurrent runtime safety are deferred to the repository-wide verification phase.

## Trách nhiệm

Quản lý vòng đời và truyền một shared-cache root cho từng benchmark scenario.

## Input và hành vi

Caller có thể truyền root đã seed; nếu không, owner tạo thư mục tạm và giữ nó tồn tại đến khi closure của scenario kết thúc. Path được truyền cho closure. Scenario tạo `WebAdapter` qua `with_registry_and_shared_cache_root`; constructor kiểm tra registry và quyền ghi cache, không dò cache từ biến môi trường.

Owner không set, remove hoặc restore biến môi trường process; không đổi metadata TTL của product. Mỗi scenario có provider/root mới trừ khi nhận cache đã seed. Cấu hình môi trường của product được giữ nguyên.

## Lỗi và test

Lỗi tạo thư mục tạm và lỗi scenario được truyền qua `anyhow::Result`; không fallback sang cache toàn cục. Test vòng đời path, cô lập scenario và an toàn khi runtime chạy đồng thời được hoãn tới pha verify toàn repository.
