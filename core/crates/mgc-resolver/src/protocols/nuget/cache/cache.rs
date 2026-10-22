//! NuGet global-packages extraction and SHA-512 sidecar materialization.
//! Giải nén global-packages NuGet và materialize sidecar SHA-512.

use crate::protocols::ResolvedEntry;
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
        let dir = nuget_root
            .join(entry.name.to_lowercase())
            .join(entry.version.to_lowercase());
        std::fs::create_dir_all(&dir)?;
        let nupkg_name = format!("{}.{}.nupkg", entry.name, entry.version);
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
