# Resolve compatibility exports / Export tương thích resolve

Re-exports the moved shared JavaScript resolver helpers so existing tests and callers retain the crate-root names while the implementation owner stays `mgc-js-service::resolve`.

Tái export helper resolver JavaScript dùng chung để giữ tên crate-root cho tests/callers; implementation thật vẫn ở `mgc-js-service::resolve`.
