//! Owns package metadata and version reads.
//! Sở hữu đọc metadata package và danh sách version.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn get_package(&self, name: &str) -> Result<Option<crate::model::Package>> {
        let row = sqlx::query(
            r#"
            SELECT name, description, dist_tags, maintainers, time, private
            FROM packages WHERE name = ?
        "#,
        )
        .bind(name)
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            let mut pkg = crate::model::Package {
                name: row.get("name"),
                description: row.get("description"),
                versions: std::collections::HashMap::new(),
                dist_tags: serde_json::from_str(&row.get::<String, _>("dist_tags"))?,
                maintainers: serde_json::from_str(&row.get::<String, _>("maintainers"))?,
                time: serde_json::from_str(&row.get::<String, _>("time"))?,
                private: row.get("private"),
            };
            // Load versions
            let versions = self.get_package_versions(name).await?;
            pkg.versions = versions;
            Ok(Some(pkg))
        } else if let Some(upstream) = &self.upstream {
            // ITEM 4: miss → fetch upstream → cache local (private store giữ public mirror)
            match upstream.fetch_json(name).await {
                Ok(Some(json)) => match serde_json::from_value::<crate::model::Package>(json) {
                    Ok(pkg) => {
                        let _ = self.put_package(&pkg).await;
                        Ok(Some(pkg))
                    }
                    Err(_) => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_package_versions(
        &self,
        name: &str,
    ) -> Result<std::collections::HashMap<String, crate::model::PackageVersion>> {
        let rows = sqlx::query(
            r#"
            SELECT version, data FROM package_versions WHERE package_name = ?
        "#,
        )
        .bind(name)
        .fetch_all(&self.db)
        .await?;

        let mut versions = std::collections::HashMap::new();
        for row in rows {
            let version: String = row.get("version");
            let data: String = row.get("data");
            let ver: crate::model::PackageVersion = serde_json::from_str(&data)?;
            versions.insert(version, ver);
        }
        Ok(versions)
    }
}
