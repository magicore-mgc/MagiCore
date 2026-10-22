# Dependency metadata policy / Policy metadata dependency

`dependency_metadata.rs` validates PubSpec dependency names, records SDK-owned packages and rejects `path`, `git`, `hosted` or other non-registry sources rather than dropping graph edges. It maps typed catalog failures to the established `MgError` contract. Its two nearest unit cases are in `test/pub_metadata.rs`; the public API integration suite remains at crate level.
