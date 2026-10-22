# Materialize tests / Test materialize

`pub_cache_layout.rs` verifies the public materialization path and extracted contents using an in-memory archive. Archive traversal, link and destination safety belong to the shared archive owner and are tested there.

`pub_cache_layout.rs` kiểm tra đường dẫn materialize public và nội dung giải nén bằng archive trong bộ nhớ. An toàn traversal, link và đích giải nén thuộc owner archive dùng chung và được test tại đó.
