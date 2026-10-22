# State access / Truy cập state

Owns poison-recovering guards for the adapter’s existing-version map and prefetch task handle.

Sở hữu guard phục hồi khi mutex poison cho map version hiện có và handle prefetch. State fields remain owned by `adapter.rs`; no state semantics are changed.
