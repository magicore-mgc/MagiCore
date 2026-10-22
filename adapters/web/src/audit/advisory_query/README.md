Owns the existing npm bulk advisory HTTP request and response path. It keeps transport/status failures as scanner-failed reports, while client/endpoint/JSON/advisory errors retain the current hard-error mapping. The shared `mgc-js-service` request-body, endpoint and parser remain the only primitives used.

Sở hữu request/response HTTP advisory bulk npm hiện có. Owner giữ lỗi transport/status thành scanner-failed report; lỗi dựng client/endpoint/JSON/advisory vẫn ánh xạ theo hợp đồng hard-error hiện tại. Chỉ dùng các primitive request-body, endpoint và parser từ `mgc-js-service`.
