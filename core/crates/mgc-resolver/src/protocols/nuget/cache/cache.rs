//! NuGet global-packages extraction and SHA-512 sidecar materialization.
//! Giải nén global-packages NuGet và materialize sidecar SHA-512.

use crate::protocols::ResolvedEntry;
use crate::protocols::cache_segment::validate_cache_segment;
use crate::protocols::nuget::NuGetProtocol;
use base64::Engine as _;
use mgc_types::MgResult;
use sha2::{Digest, Sha512};
use std::path::{Path, PathBuf};

impl NuGetProtocol {
    /// Materialize a package into the global-packages layout
    /// `{nuget_root}/{id-lower}/{version-lower}/`: the nupkg bytes, the
    /// extracted package contents and the `.nupkg.sha512` sidecar — the
    /// layout `dotnet restore --source`/global fallback folders read.
    /// Materialize package vào layout global-packages
    /// `{nuget_root}/{id-lower}/{version-lower}/`: byte nupkg, nội dung
    /// package đã giải nén và sidecar `.nupkg.sha512` — layout mà
    /// `dotnet restore --source`/global fallback folder đọc.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        nupkg_bytes: &[u8],
        nuget_root: &Path,
    ) -> MgResult<PathBuf> {
        // SEC-N1: the id comes from a manifest/registration and the version
        // from remote JSON; either one containing `..` or a separator would
        // place the global-packages directory outside `nuget_root` and every
        // later write (nupkg, nuspec rename, sha512) would follow it.
        // (SEC-N1: id từ manifest/registration và version từ JSON từ xa; nếu
        // hoặc chứa `..` hoặc dấu phân cách thì thư mục global-packages nằm
        // ngoài `nuget_root` và mọi lần ghi sau đó sẽ đi theo.)
        // NuGet's on-disk convention lowercases the DIRECTORY segment, while the
        // archive keeps the id's original casing. Validate the raw registry
        // value first (that is what may contain `..` or a separator), then
        // lowercase for the path — the two must not be conflated.
        // (Quy ước trên đĩa của NuGet hạ chữ thường cho THƯ MỤC, còn file nén
        // giữ đúng chữ hoa/thường của id. Validate giá trị gốc từ registry
        // trước — đó là thứ có thể chứa `..` hay dấu phân cách — rồi hạ chữ
        // thường cho đường dẫn; không được gộp hai việc này.)
        let id_segment = validate_cache_segment(&entry.name.to_lowercase(), "NuGet package id")?;
        let version_segment =
            validate_cache_segment(&entry.version.to_lowercase(), "NuGet package version")?;
        let dir = nuget_root.join(&id_segment).join(&version_segment);
        std::fs::create_dir_all(&dir)?;
        // The sidecar file name is also built from registry fields; a segment
        // containing `/` was already refused above, but the joined name must
        // still be exactly one component inside `dir`.
        // (Tên file sidecar cũng dựng từ trường registry; segment chứa `/` đã bị
        // từ chối, nhưng tên ghép vẫn phải là đúng một thành phần trong `dir`.)
        // The archive keeps the id's ORIGINAL casing (`Demo.Lib.13.0.3.nupkg`)
        // while the containing directory uses the lowercased segment, matching
        // NuGet's global-packages layout.
        // (File nén giữ NGUYÊN chữ hoa/thường của id
        // (`Demo.Lib.13.0.3.nupkg`) còn thư mục chứa dùng segment chữ thường,
        // đúng layout global-packages của NuGet.)
        let archive_id = validate_cache_segment(&entry.name, "NuGet package id")?;
        let nupkg_name = format!("{archive_id}.{version_segment}.nupkg");
        std::fs::write(dir.join(&nupkg_name), nupkg_bytes)?;
        crate::protocols::zip_reader::extract_zip(nupkg_bytes, &dir)?;
        // NuGet on-disk convention lowercases the id: the nuspec inside a
        // nupkg keeps author case (`Demo.Lib.nuspec`) but the global-packages
        // layout reads `<lower-id>.nuspec` — normalize so Linux checkouts
        // see the same file Windows does (case-sensitivity parity).
        // (Chuẩn on-disk của NuGet viết thường id.)
        let want_nuspec = dir.join(format!("{}.nuspec", entry.name.to_lowercase()));
        if !want_nuspec.is_file() {
            let have: Vec<_> = std::fs::read_dir(&dir)
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.extension()
                        .and_then(|x| x.to_str())
                        .is_some_and(|x| x.eq_ignore_ascii_case("nuspec"))
                })
                .collect();
            if let [single] = have.as_slice() {
                let _ = std::fs::rename(single, &want_nuspec);
            }
        }
        let sha512 = Sha512::digest(nupkg_bytes);
        std::fs::write(
            dir.join(format!("{nupkg_name}.sha512")),
            base64::engine::general_purpose::STANDARD.encode(sha512),
        )?;
        Ok(dir.join(nupkg_name))
    }
}
