# Release pre-publish scripts / Script kiểm tra trước phát hành

This folder owns fail-closed validation of the expected archive/checksum/SBOM set, GitHub attestations, and SBOM contents. The workflow owns runner permissions, artifact download, provenance generation and step order.

Folder này sở hữu việc xác minh fail-closed bộ archive/checksum/SBOM kỳ vọng, GitHub attestation và nội dung SBOM. Workflow sở hữu quyền runner, download artifact, tạo provenance và thứ tự các bước.

The checked-out root is the working directory. Version and repository owner enter through explicit environment variables. Add positive and negative nearest tests in the global test phase; this batch deliberately does not execute them.
