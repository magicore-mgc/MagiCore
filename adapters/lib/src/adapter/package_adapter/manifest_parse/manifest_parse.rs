//! Read-only language manifest dispatch for LibAdapter.
//! Điều phối parse manifest chỉ đọc cho LibAdapter.

use crate::adapter::types::LibAdapter;
use crate::language::LibLanguage;
use crate::manifest::{
    parse_cargo_manifest, parse_csproj_manifest, parse_go_mod_manifest, parse_maven_manifest,
    parse_pyproject_manifest,
};
use mgc_types::PackageAdapter;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

pub(in crate::adapter::package_adapter) async fn parse_manifest(
    adapter: &LibAdapter,
    project_root: &Path,
) -> MgResult<Manifest> {
    adapter.require_python_native_at(project_root, "parse_manifest")?;
    if let Some(web) = &adapter.web {
        return web.parse_manifest(project_root).await;
    }
    match adapter.language {
        LibLanguage::Rust => parse_cargo_manifest(project_root),
        LibLanguage::Python => parse_pyproject_manifest(project_root),
        // Go has no mgc-written manifest — `go mod` owns go.mod
        // (parse via the go list wrapper when needed).
        // Go không có manifest do mgc viết — `go mod` sở hữu go.mod.
        LibLanguage::Go => parse_go_mod_manifest(project_root),
        // Java (Phase 2): pom.xml projects are parsed by the SAME POM
        // parser the native Maven engine uses; gradle projects keep an
        // honest empty manifest (resolve fails closed downstream).
        // (Java (Phase 2): project pom.xml được parse bằng CÙNG parser
        // POM của engine Maven native; project gradle giữ manifest rỗng
        // trung thực (resolve fail-closed phía sau).)
        LibLanguage::Java => parse_maven_manifest(project_root),
        // .NET (Phase 2): the csproj's `<PackageReference>` entries feed
        // the native NuGet engine; packages.lock.json keeps owning audit
        // pin truth. mgc never rewrites the csproj (read-only parse).
        // (.NET (Phase 2): các `<PackageReference>` của csproj nạp cho
        // engine NuGet native; packages.lock.json vẫn giữ truth ghim cho
        // audit. mgc không bao giờ viết lại csproj (parse chỉ đọc).)
        LibLanguage::DotNet => parse_csproj_manifest(project_root),
        LibLanguage::Ts => unreachable!("ts handled by web delegate"),
    }
}
