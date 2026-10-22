# OCI manifest channel / Kênh OCI manifest

The `oci_reference/` owner parses `oci://host/repo[:tag]`; retrieval pulls the OCI manifest, then the artifact-validation seam requires the policy config media type and empty layers before verify/cache through the shared owner. Public parser and refresh routes remain re-exported through `commands::policy`.

Security follow-ups: `mgc-oci` buffers the complete blob before policy size validation, and the current `127.` prefix can classify non-IP hostnames as loopback. Neither issue is fixed in S207.
