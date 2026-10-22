//! Validates and converts digest encodings into safe storage path fragments.
//! Kiểm tra và chuyển digest thành các đoạn path lưu trữ an toàn.

use super::super::prelude::*;

/// Hex fragment của digest để làm path component an toàn.
/// - "sha512-<b64>": decode base64 → hex
/// - chuỗi hex thuần (vd "sha256:<hex>", hoặc hex không prefix): dùng trực tiếp
///
/// (Không slice raw b64 — b64 có thể bắt đầu `/`/`+` khiến Path::join tạo path
/// tuyệt đối, vd join("/+") → "/+/..." → tạo thư mục ở root → 500.)
///
/// Fail-closed: malformed digest (empty body, bad hex/b64) trả Err — không
/// bao giờ decode lỗi thành empty rồi lưu path rác.
pub(in crate::storage) fn digest_hex_path(digest: &str) -> Result<(String, String)> {
    const MIN_DIGEST_HEX: usize = 8;

    let (algo, body) = match digest.rfind(['-', ':']) {
        Some(i) => (&digest[..i], &digest[i + 1..]),
        None => ("", digest),
    };
    if body.is_empty() {
        anyhow::bail!("malformed digest '{digest}': empty body");
    }
    let bytes = if body.bytes().all(|c| c.is_ascii_hexdigit()) {
        if body.len() % 2 != 0 {
            anyhow::bail!("malformed digest '{digest}': odd-length hex body");
        }
        (0..body.len() / 2)
            .map(|i| {
                u8::from_str_radix(&body[i * 2..i * 2 + 2], 16)
                    .map_err(|_| anyhow::anyhow!("malformed digest '{digest}': bad hex pair"))
            })
            .collect::<Result<Vec<u8>>>()?
    } else {
        let algo_prefix = algo.split(':').next().unwrap_or("sha512");
        if !algo_prefix.contains("sha") {
            anyhow::bail!("malformed digest '{digest}': unknown algorithm '{algo_prefix}'");
        }
        base64::engine::general_purpose::STANDARD
            .decode(body)
            .map_err(|_| anyhow::anyhow!("malformed digest '{digest}': invalid base64 body"))?
    };
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    if hex.len() < MIN_DIGEST_HEX {
        anyhow::bail!("malformed digest '{digest}': shorter than {MIN_DIGEST_HEX} hex chars");
    }
    Ok((hex[..2].to_string(), hex[2..].to_string()))
}
