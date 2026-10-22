# Generation-token support

Owns fixtures and test-only assertions shared by this Cargo integration-test target. `support.rs` contains `TestDb`, project-root constants, and the direct SQL assertion that SQLite rejects orphan CAS claims by the composite foreign key; it does not wrap production behavior.

Sở hữu fixture và assertion test-only dùng chung trong Cargo integration-test target này. `support.rs` chứa `TestDb`, hằng project root và assertion SQL trực tiếp rằng SQLite từ chối CAS claim mồ côi bằng foreign key ghép; không bao bọc behavior production.
