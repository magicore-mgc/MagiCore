//! Build minimal in-memory ZIP archives for tests — dựng ZIP tối giản trong bộ nhớ cho test.

use super::super::crc32::crc32;

/// Build a store/deflate ZIP archive without a ZIP-writer dependency.
/// Dựng archive ZIP store/deflate mà không cần crate ghi ZIP.
pub(crate) fn build_zip(entries: &[(&str, Vec<u8>, u16)]) -> Vec<u8> {
    use std::io::Write as _;
    let mut out: Vec<u8> = Vec::new();
    let mut centrals: Vec<Vec<u8>> = Vec::new();
    for (name, data, method) in entries {
        let local_offset = out.len() as u32;
        let compressed = match *method {
            8 => {
                let mut enc =
                    flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
                enc.write_all(data).expect("test operation must succeed");
                enc.finish().expect("test operation must succeed")
            }
            _ => data.clone(),
        };
        let crc = crc32(data);
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes()); // Version — phiên bản.
        out.extend_from_slice(&0u16.to_le_bytes()); // Flags — cờ.
        out.extend_from_slice(&method.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes()); // Time/date — thời gian/ngày.
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // Extra length — độ dài trường mở rộng.
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&compressed);

        let mut cd = Vec::new();
        cd.extend_from_slice(b"PK\x01\x02");
        cd.extend_from_slice(&20u16.to_le_bytes());
        cd.extend_from_slice(&20u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&method.to_le_bytes());
        cd.extend_from_slice(&0u32.to_le_bytes());
        cd.extend_from_slice(&crc.to_le_bytes());
        cd.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        cd.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cd.extend_from_slice(&(name.len() as u16).to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes()); // Extra — trường mở rộng.
        cd.extend_from_slice(&0u16.to_le_bytes()); // Comment — ghi chú.
        cd.extend_from_slice(&0u16.to_le_bytes()); // Disk — đĩa.
        cd.extend_from_slice(&0u16.to_le_bytes()); // Internal attributes — thuộc tính nội bộ.
        cd.extend_from_slice(&0u32.to_le_bytes()); // External attributes — thuộc tính bên ngoài.
        cd.extend_from_slice(&local_offset.to_le_bytes());
        cd.extend_from_slice(name.as_bytes());
        centrals.push(cd);
    }
    let cd_offset = out.len() as u32;
    for cd in &centrals {
        out.extend_from_slice(cd);
    }
    let cd_size = out.len() as u32 - cd_offset;
    out.extend_from_slice(b"PK\x05\x06");
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(centrals.len() as u16).to_le_bytes());
    out.extend_from_slice(&(centrals.len() as u16).to_le_bytes());
    out.extend_from_slice(&cd_size.to_le_bytes());
    out.extend_from_slice(&cd_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // Comment length — độ dài ghi chú.
    out
}
