use super::error;
use anyhow::Result;

pub(in crate::commands::template) fn verify_template_integrity(
    bytes: &[u8],
    integrity: &str,
) -> Result<()> {
    use base64::Engine;
    use sha2::{Digest, Sha256, Sha384, Sha512};
    let rank = |token: &str| {
        let algorithm = token
            .split_once('-')
            .map(|(algorithm, _)| algorithm)
            .unwrap_or("");
        if algorithm.eq_ignore_ascii_case("sha512") {
            3
        } else if algorithm.eq_ignore_ascii_case("sha384") {
            2
        } else if algorithm.eq_ignore_ascii_case("sha256") {
            1
        } else {
            0
        }
    };
    let strongest = integrity
        .split_whitespace()
        .map(rank)
        .max()
        .filter(|rank| *rank > 0)
        .ok_or_else(error::template_integrity_invalid)?;
    let actual = match strongest {
        3 => Sha512::digest(bytes).to_vec(),
        2 => Sha384::digest(bytes).to_vec(),
        _ => Sha256::digest(bytes).to_vec(),
    };
    let mut saw_valid_digest = false;
    for token in integrity
        .split_whitespace()
        .filter(|token| rank(token) == strongest)
    {
        let hash = token
            .split_once('-')
            .map(|(_, hash)| hash)
            .unwrap_or("")
            .split('?')
            .next()
            .unwrap_or("");
        if hash.len() > actual.len().div_ceil(3) * 4 {
            continue;
        }
        let Ok(expected) = base64::engine::general_purpose::STANDARD
            .decode(hash)
            .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(hash))
        else {
            continue;
        };
        if expected.len() != actual.len() {
            continue;
        }
        saw_valid_digest = true;
        if expected == actual {
            return Ok(());
        }
    }
    if !saw_valid_digest {
        return Err(error::template_integrity_invalid());
    }
    Err(error::template_integrity_mismatch())
}
