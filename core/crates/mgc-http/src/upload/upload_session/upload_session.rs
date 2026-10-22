// Upload session DTO — preserve the existing public serialized fields.
// DTO phiên upload — giữ nguyên các field công khai và định dạng serialize.

/// Upload session state — trạng thái phiên upload.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadSession {
    pub upload_id: String,
    pub offset: u64,
    pub total_size: Option<u64>,
    pub digest: String, // SHA-256 for the complete blob — SHA-256 của toàn blob.
    pub uploaded_digest: String, // SHA-256 for uploaded bytes — SHA-256 của phần đã gửi.
    pub started_at: String, // ISO-8601 timestamp — thời điểm ISO-8601.
    pub updated_at: String,
    pub chunk_size: u64,
}

#[cfg(test)]
#[path = "test/upload_session.rs"]
mod tests;
