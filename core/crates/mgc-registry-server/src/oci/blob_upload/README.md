# Blob Upload

## Responsibility

OCI upload session start, ranged append and final blob publication.

## Trách nhiệm

Khởi tạo phiên upload OCI, nối chunk theo range và công bố blob cuối.

## Layout

- `blob_upload.rs`: implementation for this owner.
- `mod.rs`: module wiring and the smallest required re-export surface.
- `test/`: only present when this owner has a moved local regression test.

## Boundaries

Keep OCI route/status/header/storage behavior owned here. Cross-owner calls must use the explicit parent exports; do not duplicate validation or HTTP mapping.

## Ranh giới

Giữ hành vi route/status/header/storage của OCI tại owner này. Gọi owner khác qua export tường minh; không sao chép logic kiểm tra hay ánh xạ HTTP.
