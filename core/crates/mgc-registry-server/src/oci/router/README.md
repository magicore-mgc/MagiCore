# Router

## Responsibility

Route matching, authorization dispatch, OCI v2 ping and routing.

## Trách nhiệm

Phân giải route, phân quyền/điều phối, ping OCI v2 và đăng ký endpoint.

## Layout

- `router.rs`: implementation for this owner.
- `mod.rs`: module wiring and the smallest required re-export surface.
- `test/`: only present when this owner has a moved local regression test.

## Boundaries

Keep OCI route/status/header/storage behavior owned here. Cross-owner calls must use the explicit parent exports; do not duplicate validation or HTTP mapping.

## Ranh giới

Giữ hành vi route/status/header/storage của OCI tại owner này. Gọi owner khác qua export tường minh; không sao chép logic kiểm tra hay ánh xạ HTTP.
