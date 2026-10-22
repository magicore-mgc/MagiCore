//! Builds the existing registry package search results from package metadata.
//! Tạo kết quả tìm package theo hành vi hiện có từ metadata package.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn search_packages(
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<crate::model::SearchResultItem>> {
        let rows = sqlx::query(
            r#"
            SELECT name, description FROM packages 
            WHERE name LIKE ? AND private = 0
            LIMIT ? OFFSET ?
        "#,
        )
        .bind(format!("%{}%", query))
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;

        let mut results = Vec::new();
        for row in rows {
            let name: String = row.get("name");
            let description: Option<String> = row.get("description");

            results.push(crate::model::SearchResultItem {
                package: crate::model::SearchPackage {
                    name,
                    version: "latest".to_string(),
                    description,
                    keywords: None,
                    date: chrono::Utc::now().to_rfc3339(),
                    links: crate::model::SearchLinks {
                        npm: None,
                        homepage: None,
                        repository: None,
                        bugs: None,
                    },
                    publisher: crate::model::SearchPublisher {
                        username: "registry".to_string(),
                        email: None,
                    },
                },
                score: 1.0,
                search_score: 1.0,
            });
        }
        Ok(results)
    }
}
