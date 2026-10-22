# WebAdapter composition / Composition WebAdapter

Owns the state, construction, and capability composition for the public web adapter. The crate API remains `mgc_web_adapter::WebAdapter`; `adapter.rs` defines that type and `mod.rs` only wires responsibility owners.

Sở hữu state, khởi tạo và composition capability của adapter web. API crate vẫn là `mgc_web_adapter::WebAdapter`; `adapter.rs` định nghĩa type, `mod.rs` chỉ nối các owner.

Behavior is delegated to constructor, age-policy, package-selection, package-operation, resolution, fetch/store, and audit owners. Existing tests stay in `src/test/` for this source-first pass; moving/adding nearest-owner tests is deferred to the verification phase. No local error enum is added: existing `MgError` and constructor error types remain authoritative.
