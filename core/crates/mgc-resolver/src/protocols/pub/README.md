# pub.dev resolver / Resolver pub.dev

`protocols::pubdev` is the stable public module path. `pub.rs` owns protocol state, constructors and default configuration. `registry_client/` owns HTTP metadata/archive requests; `materialize/` owns the Dart cache layout; `package_catalog/` owns registry metadata and source policy; `version_constraints/` owns Dart range matching; `graph_resolution/` owns bounded joint-root selection; `registry/` implements the public `RegistryProtocol` contract.

`protocols::PubProtocol` and `protocols::pubdev::PubProtocol` remain available. Typed catalog/graph errors map to the existing `MgError` variants. Request concurrency, selection order, archive digest checks, error messages and cache layout are preserved. This resolver does not claim PubGrub backtracking.

Unit tests are registered under nearest owners; network/integration behavior remains in `core/crates/mgc-resolver/tests/protocols_pub.rs`. Tests have not been run during source decomposition.

Mục tiêu mở rộng là tách transport, cache layout, policy catalog, constraint Dart, solver nhiều root và boundary trait mà không đổi API/hành vi. Không tuyên bố backtracking PubGrub. Test unit nằm cạnh owner; integration network giữ tại crate.
