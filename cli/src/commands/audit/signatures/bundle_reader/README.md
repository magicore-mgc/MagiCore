# Bundle reader

Streams one registry signature page and enforces the existing 16 MiB byte ceiling against declared and observed lengths before JSON decoding. Stream and JSON failures retain a typed internal source and map to the existing stable CLI error text.

Implementation: `bundle_reader.rs`; local error mapping: `error.rs`; tests: `test/size_limit.rs`. The page is still buffered up to the configured ceiling for JSON decoding.
