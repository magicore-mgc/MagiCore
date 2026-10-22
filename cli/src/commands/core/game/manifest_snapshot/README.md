# Manifest snapshot / Snapshot manifest

Owns exact Cargo.toml snapshot reads and rollback writes through the existing checked atomic writer. It preserves typed error and causal I/O source behavior.

Owner này đọc snapshot Cargo.toml và rollback bằng atomic writer đã có. Giữ typed error và source chain I/O.
