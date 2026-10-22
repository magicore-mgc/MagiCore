# Maven RegistryProtocol implementation / Triển khai RegistryProtocol Maven

This owner implements Maven metadata resolution, POM dependency parsing, artifact checksum selection, and download through the shared `RegistryProtocol` contract. The `#[async_trait]` attribute belongs directly on this implementation. Existing integration tests remain the full resolve/download/materialize coverage.

Owner này triển khai resolve metadata Maven, parse dependency trong POM, chọn checksum artifact và tải qua hợp đồng `RegistryProtocol` dùng chung. Attribute `#[async_trait]` được gắn trực tiếp vào implementation này. Integration tests hiện có tiếp tục bao phủ luồng resolve/download/materialize đầy đủ.
