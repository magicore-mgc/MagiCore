# OCI reference owner / Owner OCI reference

Parse the `oci://host[/prefix]/repo[:tag]` grammar and choose HTTPS or current loopback HTTP behavior. Public parser visibility remains available through `commands::policy::parse_oci_reference`.

Phân tích grammar OCI và giữ route public. Nearest tests cover registry, repo, tag, default tag and malformed inputs. Current string-prefix loopback rule is a separately recorded security finding.
