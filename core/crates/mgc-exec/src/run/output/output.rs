//! Bounded child-output conversion and diagnostic-tail formatting.
//! Chuyển đổi output process con có giới hạn và định dạng phần log chẩn đoán.

use std::collections::VecDeque;
use std::io::Read;

#[path = "error.rs"]
mod error;
pub(in crate::run::implementation) use error::StreamReadError;

/// Maximum bytes retained from each captured stdout/stderr stream.
/// Số byte tối đa giữ lại từ mỗi luồng stdout/stderr đã capture.
const MAX_CAPTURE_BYTES: usize = 1024 * 1024;

/// Maximum lines retained in diagnostic tails.
/// Số dòng tối đa giữ trong phần tail chẩn đoán.
const MAX_CAPTURE_LINES: usize = 40;

/// Read chunk size used while draining a child pipe.
/// Kích thước chunk khi đọc liên tục pipe của process con.
const STREAM_READ_CHUNK_BYTES: usize = 8 * 1024;

/// Drain a child pipe to EOF while retaining only its bounded trailing bytes.
/// Đọc pipe tới EOF nhưng chỉ giữ lại phần byte cuối có giới hạn.
pub(in crate::run::implementation) fn read_stream<R: Read>(
    mut reader: R,
) -> Result<Vec<u8>, StreamReadError> {
    let mut retained = VecDeque::new();
    let mut chunk = [0; STREAM_READ_CHUNK_BYTES];
    loop {
        let read = match reader.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => read,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(source) => return Err(StreamReadError::new(source)),
        };
        if read >= MAX_CAPTURE_BYTES {
            retained.clear();
            retained.extend(chunk[read - MAX_CAPTURE_BYTES..read].iter().copied());
            continue;
        }
        let excess = retained
            .len()
            .saturating_add(read)
            .saturating_sub(MAX_CAPTURE_BYTES);
        if excess > 0 {
            drop(retained.drain(..excess));
        }
        retained.extend(chunk[..read].iter().copied());
    }

    let (first, second) = retained.as_slices();
    let mut bytes = Vec::with_capacity(retained.len());
    bytes.extend_from_slice(first);
    bytes.extend_from_slice(second);
    Ok(bytes)
}

/// Convert the final bounded byte/line window into a diagnostic tail.
/// Chuyển cửa sổ byte/dòng cuối cùng thành tail để hiển thị lỗi.
pub(in crate::run::implementation) fn tail(bytes: &[u8]) -> String {
    let start = bytes.len().saturating_sub(MAX_CAPTURE_BYTES);
    let slice = &bytes[start..];
    let text = String::from_utf8_lossy(slice);
    let mut lines: Vec<&str> = text.lines().collect();
    if lines.len() > MAX_CAPTURE_LINES {
        lines = lines.split_off(lines.len() - MAX_CAPTURE_LINES);
    }
    lines.join("\n")
}

/// Convert the final byte-bounded window without applying the diagnostic line cap.
/// Chuyển cửa sổ byte cuối cùng nhưng không áp giới hạn dòng dành cho log lỗi.
pub(in crate::run::implementation) fn full(bytes: &[u8]) -> String {
    let slice = if bytes.len() > MAX_CAPTURE_BYTES {
        &bytes[bytes.len() - MAX_CAPTURE_BYTES..]
    } else {
        bytes
    };
    String::from_utf8_lossy(slice).to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
