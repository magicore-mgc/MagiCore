# Bare request validation

Owns the existing preflight sequence for global add, install dry-run, and offline-install gates. The current per-core allowlists and fail-closed diagnostics are frozen; the shared `BareRouteError` stays in the parent because operation mappers also use it.

# Tiền kiểm bare request

Sở hữu thứ tự preflight hiện tại cho global add, dry-run install và offline install. Allowlist từng core cùng diagnostic fail-closed được giữ nguyên; `BareRouteError` vẫn ở owner cha vì các mapper thao tác khác cũng dùng nó.
