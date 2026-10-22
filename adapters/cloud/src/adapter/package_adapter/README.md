# Package Adapter owner

Owns the stable `PackageAdapter for CloudAdapter` boundary. Its small methods delegate to the `project_metadata/`, `web_delegation/`, `manifest/` and `listing/` behavior owners. Unsupported Cloudflare/Terraform dependency operations remain unsupported.

Sở hữu ranh giới ổn định `PackageAdapter for CloudAdapter`. Các method ngắn chuyển sang owner hành vi `project_metadata/`, `web_delegation/`, `manifest/` và `listing/`. Thao tác dependency Cloudflare/Terraform chưa hỗ trợ tiếp tục giữ nguyên.

This folder's facade wires the trait implementation and behavior owners. No package capability or operation is introduced here.

Facade của folder nối trait implementation với các behavior owner. Không bổ sung capability hay thao tác package mới.
