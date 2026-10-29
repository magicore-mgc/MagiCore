//! Decompressed ZIP entry DTO — DTO entry ZIP đã giải nén.

/// One decompressed ZIP entry — một entry ZIP đã giải nén.
#[derive(Debug, Clone)]
pub struct ZipEntry {
    /// Stored archive name; raw names are never path-joined.
    /// Tên trong archive; không ghép path trực tiếp từ tên thô.
    pub name: String,
    /// Decompressed payload — payload đã giải nén.
    pub data: Vec<u8>,
    /// External UNIX mode bits carried by the central directory, when present.
    /// Bit mode UNIX bên ngoài do central directory mang, khi có.
    pub unix_mode: Option<u32>,
}

#[cfg(test)]
#[path = "test/zip_entry.rs"]
mod tests;
