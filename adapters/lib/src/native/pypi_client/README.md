# PyPI client

EN: Owns the Lib adapter's native PyPI JSON client. `mod.rs` preserves `native::pypi_client::PyPiClient`; `pypi_client.rs` coordinates the stable `RegistryClient` contract. Child owners handle response DTOs, metadata projection, metadata request/status policy, release selection, artifact download and typed internal errors.

VI: Sở hữu PyPI JSON client native của Lib adapter. `mod.rs` giữ đường dẫn `native::pypi_client::PyPiClient`; `pypi_client.rs` điều phối contract `RegistryClient` hiện tại. Owner con xử lý DTO response, metadata, request/status, chọn release, tải artifact và lỗi nội bộ có kiểu.

## Existing contract and limits

The default registry, URL formatting, release sorting, wheel-before-sdist choice, existing status behavior, public `MgError::Other` boundary and diagnostic strings are retained. This structural batch does not add URL restrictions, redirect policy, response-size limits, retries or caching. Those runtime/security behaviors remain separate follow-up work. Existing mock HTTP integration tests remain at `adapters/lib/tests/native_tests.rs`; nearest-owner test placement is deferred until the all-source refactor phase closes, as requested.

Registry mặc định, cách tạo URL, sắp xếp release, ưu tiên wheel, status behavior, ranh giới `MgError::Other` và diagnostic hiện có được giữ nguyên. Batch cấu trúc này không thêm chặn URL, redirect policy, giới hạn response, retry hay cache. Những hành vi runtime/security đó là đầu việc riêng. Test HTTP mock hiện tại giữ ở `adapters/lib/tests/native_tests.rs`; test gần owner sẽ được xử lý sau khi hoàn tất pha refactor source toàn repo theo yêu cầu.
