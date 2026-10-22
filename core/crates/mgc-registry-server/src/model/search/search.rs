//! NPM search response graph.
//! Cây response tìm kiếm NPM.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub objects: Vec<SearchResultItem>,
    pub total: u64,
    pub time: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResultItem {
    pub package: SearchPackage,
    pub score: f64,
    #[serde(rename = "searchScore")]
    pub search_score: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub date: String,
    pub links: SearchLinks,
    pub publisher: SearchPublisher,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchLinks {
    pub npm: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub bugs: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchPublisher {
    pub username: String,
    pub email: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
