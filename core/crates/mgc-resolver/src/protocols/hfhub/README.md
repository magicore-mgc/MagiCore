# Hugging Face Hub resolver

This folder implements a model-artifact protocol, not a package RegistryProtocol. The root module is a compatibility facade. Responsibilities are split into protocol configuration, public DTOs, identity validation, tree decoding, HTTP resolve/download operations, and file integrity. Public paths remain protocols::hfhub::* and protocols::{HfHubProtocol, ModelFile, ModelResolution, git_blob_sha1}.

Thư mục này triển khai protocol artifact model, không phải RegistryProtocol cho package. Module gốc là facade tương thích. Trách nhiệm được tách thành cấu hình protocol, DTO công khai, kiểm tra định danh, parse tree, thao tác HTTP resolve/download và toàn vẹn file. Giữ đường công khai protocols::hfhub::* và protocols::{HfHubProtocol, ModelFile, ModelResolution, git_blob_sha1}.

The unchanged crate integration suite is core/crates/mgc-resolver/tests/protocols_hfhub.rs. This module has no production caller in core/cli/adapters in the current source map; the refactor does not claim end-to-end model installation. All tests remain NOT_RUN until the repository-wide source pass closes and the disk gate is satisfied.

Integration suite của crate giữ nguyên là core/crates/mgc-resolver/tests/protocols_hfhub.rs. Source map hiện tại không có production caller trong core/cli/adapters; refactor này không tuyên bố cài model end-to-end. Mọi test vẫn NOT_RUN đến khi pass source toàn repo khép lại và điều kiện ổ đĩa đạt.
