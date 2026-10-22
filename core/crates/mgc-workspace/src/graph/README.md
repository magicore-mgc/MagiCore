# Workspace graph / Graph workspace

This owner preserves the public graph model and package-manifest reader exported by `mgc-workspace`. It coordinates graph values and delegates Node, Rust, and Python manifest parsing to language-specific owners. Topology and workspace discovery stay in their existing owners.

Owner contract: keep all `mgc_workspace::*` exports, public fields, manifest priority and fallback semantics stable. This folder does not add support for frameworks beyond the three manifest formats currently present in source.
