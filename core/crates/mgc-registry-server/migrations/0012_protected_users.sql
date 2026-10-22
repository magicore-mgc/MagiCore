-- Stage protected credentials separately to avoid raw-token/digest collisions.
-- Tách bảng staging để token plaintext không va chạm digest trong lúc chuyển.
CREATE TABLE users_secure (
    name TEXT PRIMARY KEY,
    token_digest TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    email TEXT,
    is_admin BOOLEAN NOT NULL DEFAULT 0,
    scopes TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    role TEXT NOT NULL DEFAULT 'viewer'
);
