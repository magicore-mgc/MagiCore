# http

EN: TLS HTTP status/body/retry policy. `HttpError::Network` owns transport and response failures, then maps to the shared `MgError::Network` category at the protocol boundary.

VI: Policy TLS HTTP, status/body/retry. `HttpError::Network` sở hữu lỗi transport/response rồi chuyển thành nhóm `MgError::Network` tại boundary protocol.
