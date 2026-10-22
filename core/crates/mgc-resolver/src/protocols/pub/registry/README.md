# Registry protocol boundary / Boundary registry

`registry.rs` implements `RegistryProtocol` for `PubProtocol`, forwarding single-package selection, joint-root resolution, requirement matching and archive download. It owns no separate error taxonomy; the existing `MgError` contract remains at the public trait boundary.

`mod.rs` only registers implementation. External adapter callers remain on `mgc_resolver::protocols::PubProtocol` or `protocols::pubdev::PubProtocol`.
