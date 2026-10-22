# SPDX serializer

This crate-private facade keeps `crate::spdx::serialize` stable. Child owners validate the request, project package identities/checksums/relationships, assemble SPDX DTOs, and convert local errors once at the boundary.
