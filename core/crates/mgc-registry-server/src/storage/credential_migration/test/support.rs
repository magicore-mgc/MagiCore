use crate::auth::{User, UserRole};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::{borrow::Cow, path::Path};

pub(super) async fn legacy_database(directory: &Path) -> Pool<Sqlite> {
    let pool = SqlitePool::connect(&database_url(directory)).await.unwrap();
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.migrations = Cow::Owned(
        migrations
            .iter()
            .filter(|migration| migration.version < 12)
            .cloned()
            .collect(),
    );
    migrations.run(&pool).await.unwrap();
    pool
}

pub(super) fn database_url(directory: &Path) -> String {
    format!(
        "sqlite://{}?mode=rwc",
        directory.join("registry.db").display()
    )
}

pub(super) fn user(name: &str, password: Option<&str>) -> User {
    User {
        name: name.into(),
        password: password.map(str::to_owned),
        email: Some("account@example.test".into()),
        is_admin: false,
        role: UserRole::Publisher,
        scopes: vec!["@example/*".into()],
    }
}

pub(super) async fn insert_legacy(pool: &Pool<Sqlite>, token: &str, user: &User) {
    sqlx::query("INSERT INTO users(name, token, password, email, is_admin, scopes, role, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, '2026-01-02 03:04:05')")
        .bind(&user.name).bind(token).bind(&user.password).bind(&user.email)
        .bind(user.is_admin).bind(serde_json::to_string(&user.scopes).unwrap())
        .bind(user.role.as_str()).execute(pool).await.unwrap();
}
