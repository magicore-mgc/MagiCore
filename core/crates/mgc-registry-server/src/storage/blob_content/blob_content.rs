//! Owns package blob reads/writes, digest verification, and upstream tarball fetch.
//! Sở hữu đọc/ghi package blob, xác minh digest và lấy tarball upstream.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Recompute the digest from the payload and refuse mismatches.
    /// Supports the two wire formats this registry serves:
    /// - "sha512-<base64>" (npm integrity style)
    /// - "sha256:<hex>" / "<hex>" (PyPI/OCI style)
    ///
    /// The stored digest is the SERVER-computed one; a client declaring
    /// digest A while uploading content B is rejected (P0-2).
    ///
    /// Tính lại digest từ payload và từ chối khi lệch. Server là nguồn chân lý
    /// của digest — client khai A nhưng upload B sẽ bị từ chối.
    pub(in crate::storage) fn compute_wire_digest(digest: &str, data: &[u8]) -> Result<String> {
        use sha2::{Digest, Sha256, Sha512};

        let (algo, _body) = match digest.rfind(['-', ':']) {
            Some(i) => (&digest[..i], &digest[i + 1..]),
            None => ("", digest),
        };

        if algo.contains("512") {
            let mut hasher = Sha512::new();
            hasher.update(data);
            let b64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                hasher.finalize(),
            );
            Ok(format!("sha512-{b64}"))
        } else {
            // sha256:<hex>, bare hex, or unknown algo — sha256 is the default
            // server-side truth (PyPI/OCI both use it).
            let mut hasher = Sha256::new();
            hasher.update(data);
            Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
        }
    }

    pub async fn put_blob(&self, digest: &str, data: &[u8]) -> Result<()> {
        let (p1, p2) = digest_hex_path(digest)?;
        let key = format!("{p1}/{p2}");

        // Server-side digest recomputation: the declared digest must match
        // the uploaded bytes exactly. A client declaring digest A while
        // uploading content B is rejected (P0-2) — provenance and
        // content-addressing stay trustworthy.
        // Server tính lại digest: digest khai phải khớp CHÍNH XÁC bytes
        // upload. Client khai A nhưng upload B bị từ chối (P0-2) — giữ
        // provenance và content-addressing đáng tin.
        let actual = Self::compute_wire_digest(digest, data)?;
        if actual != digest {
            anyhow::bail!("digest mismatch: declared '{digest}' but bytes hash to '{actual}'");
        }

        self.backend.put(&key, data).await?;

        // Vòng-2 review R-put: if the final path pre-existed (lost race, or
        // a torn leftover from an old crash), `put` may have skipped the
        // overwrite — verify the PUBLISHED file's length matches the bytes
        // we intended to store, so a torn/stale blob can never be recorded
        // in the DB as this digest.
        // (R-put vòng-2 review: nếu path cuối đã tồn tại trước (thua race,
        // hoặc rác cụt từ crash cũ), `put` có thể đã bỏ qua ghi đè — verify
        // độ dài file ĐÃ PUBLISH khớp bytes định lưu, để blob cụt/cũ không
        // bao giờ được DB ghi nhận là digest này.)
        self.backend.verify_published_len(&key, data.len()).await?;

        sqlx::query(
            r#"
            INSERT INTO blobs (digest, size, path)
            VALUES (?, ?, ?)
            ON CONFLICT(digest) DO UPDATE SET size = excluded.size, path = excluded.path
        "#,
        )
        .bind(digest)
        .bind(data.len() as i64)
        .bind(key)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_blob(&self, digest: &str) -> Result<Option<Vec<u8>>> {
        // Lookup by the canonical server-computed digest: same algorithm as
        // put_blob, so a declared digest resolves to the stored row.
        // Tra theo digest chuẩn do server tính: cùng thuật toán với put_blob
        // nên digest khai từ client vẫn trỏ đúng row đã lưu.
        let row = sqlx::query("SELECT path FROM blobs WHERE digest = ?")
            .bind(digest)
            .fetch_optional(&self.db)
            .await?;

        if let Some(row) = row {
            let key: String = row.get("path");
            let data = self.backend.get(&key).await?;
            if let Some(data) = data {
                // Trust-but-verify: serving path re-checks the bytes against
                // the declared digest — a corrupted/mismatched store entry
                // fails closed instead of serving poison.
                // Kiểm tra lại khi phục vụ: bytes phải khớp digest khai —
                // entry hỏng/sai sẽ fail cứng thay vì phát dữ liệu bẩn.
                let actual = Self::compute_wire_digest(digest, &data)?;
                if actual != digest {
                    anyhow::bail!(
                        "blob integrity mismatch for '{digest}': bytes hash to '{actual}'"
                    );
                }
                return Ok(Some(data));
            }
            return Ok(None);
        }
        Ok(None)
    }

    /// Fetch tarball từ upstream (ITEM 4). None khi chưa cấu hình upstream,
    /// URL không cùng host upstream (chống SSRF) hoặc miss.
    pub async fn fetch_upstream_tarball(&self, tarball_url: &str) -> Result<Option<Vec<u8>>> {
        let Some(upstream) = &self.upstream else {
            return Ok(None);
        };
        let same_host = url::Url::parse(tarball_url)
            .ok()
            .and_then(|u| u.host_str().map(str::to_owned))
            .zip(
                url::Url::parse(&upstream.base)
                    .ok()
                    .and_then(|u| u.host_str().map(str::to_owned)),
            )
            .is_some_and(|(a, b)| a == b);
        if !same_host {
            return Ok(None);
        }
        upstream.fetch_bytes(tarball_url).await
    }
}
