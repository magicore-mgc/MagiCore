# Trusted publish request timeouts

## Purpose

Owns bounding one trusted-publish request by remaining token lifetime: subtract the expiry skew, fail loudly when the token expires too soon, and cap at the per-transport maximum (Docker push, OCI request, npm request).

## Vietnamese

Sở hữu việc giới hạn một request trusted-publish theo thời gian sống token còn lại: trừ skew hết hạn, báo lỗi rõ khi token sắp hết, và chặn trên theo mức tối đa từng transport (Docker push, OCI request, npm request).
