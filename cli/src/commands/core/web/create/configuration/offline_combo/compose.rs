//! Compose a split fullstack combo offline: frontend half, then server half.
//! Ghép combo fullstack tách rời offline: nửa frontend, rồi nửa server.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::super::super::*;
use crate::commands::core::create::validate_project_name;

/// Outcome of composing a combo offline.
/// Kết quả ghép combo offline.
#[derive(Debug)]
pub(in crate::commands::core::web::implementation) struct ComposedCombo {
    /// Directory MagiCore's metadata files are written into.
    /// Thư mục ghi metadata của MagiCore.
    pub(in crate::commands::core::web::implementation) project_dir: PathBuf,
    /// Provenance entries for every layer that contributed source.
    /// Entry provenance cho mọi layer đã đóng góp source.
    pub(in crate::commands::core::web::implementation) provenance: Vec<String>,
    /// Frontend framework used for the frontend half (`react-vite`, `vue-vite`).
    /// Framework frontend của nửa frontend (`react-vite`, `vue-vite`).
    pub(in crate::commands::core::web::implementation) frontend_framework: String,
}

/// Whether `combo` is a split combo whose frontend half has a scaffold source.
/// `combo` có phải combo tách rời mà nửa frontend có nguồn scaffold.
///
/// Only the split shape is composed: all-in-one frameworks (`nextjs`, `nuxt`,
/// `sveltekit`, `remix`) ship their own server routes and are not combos.
/// Chỉ ghép dạng tách rời: framework all-in-one (`nextjs`, `nuxt`, `sveltekit`,
/// `remix`) tự có route server, không phải combo.
pub(in crate::commands::core::web::implementation) fn is_combo(combo: &str) -> bool {
    backend_framework(combo).is_some()
        && !crate::scaffold::processor::is_all_in_one_fullstack(combo)
}

/// Server half of a combo, whichever owner can write it.
/// Nửa server của combo, do owner nào ghi được.
///
/// Node backends delegate to their starter catalog (a real framework
/// initializer); the rest are written from data by the native owner. Giving the
/// native owner precedence keeps each backend in exactly one catalog.
/// Backend Node giao cho catalog starter của nó (initializer framework thật);
/// phần còn lại do owner native ghi từ dữ liệu. Ưu tiên owner native bảo đảm
/// mỗi backend chỉ nằm trong đúng một catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServerHalf {
    /// Node.js starter — starter Node.js.
    Node,
    /// Native half written by `native_backend` — nửa native do `native_backend` ghi.
    Native,
}

/// Resolve who writes the server half of `backend`.
/// Xác định ai ghi nửa server của `backend`.
fn server_half(backend: &str) -> Option<ServerHalf> {
    if super::native_backend::has_half(backend) {
        Some(ServerHalf::Native)
    } else if super::node_backend::has_half(backend) {
        Some(ServerHalf::Node)
    } else {
        None
    }
}

/// Backend framework embedded in a split combo name.
/// Framework backend nằm trong tên combo tách rời.
fn backend_framework(combo: &str) -> Option<&'static str> {
    fullstack_backend_framework(combo)
}

/// Frontend framework of a split combo, derived from its own prefix.
/// Framework frontend của combo tách rời, suy từ tiền tố của chính tên.
///
/// The combo name is `<frontend>-<backend>`, so the frontend half is the name
/// with the backend suffix removed. Deriving it locally keeps the mapping in one
/// place instead of widening another module's export surface.
/// Tên combo là `<frontend>-<backend>`, nên nửa frontend là tên đã bỏ hậu tố
/// backend. Suy tại chỗ giữ ánh xạ ở một nơi thay vì mở rộng bề mặt export của
/// module khác.
fn frontend_half_framework<'a>(backend: &str, combo: &'a str) -> &'a str {
    combo
        .strip_suffix(backend)
        .map(|prefix| prefix.trim_end_matches('-'))
        .filter(|prefix| !prefix.is_empty())
        .unwrap_or(combo)
}

/// Compose `combo` offline: the frontend half's own initializer writes the
/// frontend, then the server half is written into `server/`.
/// Ghép `combo` offline: initializer của nửa frontend ghi frontend, rồi nửa
/// server được ghi vào `server/`.
///
/// The frontend half always runs its framework's published initializer, so the
/// combo never contains framework source written by MagiCore. When that
/// initializer cannot run (offline registry, no initializer) the error names
/// both halves instead of the combo leaf layer nobody can reach.
/// Nửa frontend luôn chạy initializer do framework phát hành, nên combo không bao
/// giờ chứa source framework do MagiCore viết. Khi initializer không chạy được
/// (registry offline, không có initializer), lỗi nêu rõ cả hai nửa thay vì leaf
/// combo mà không ai tiếp cận được.
///
/// The server half is refused before the frontend is scaffolded whenever no
/// owner can produce it, so a failure never leaves a frontend-only directory
/// that would look finished.
/// Nửa server bị từ chối trước khi scaffold frontend nếu không owner nào tạo
/// được, nên lỗi không bao giờ để lại thư mục chỉ có frontend trông như xong.
pub(in crate::commands::core::web::implementation) async fn compose_offline(
    combo: &str,
    project_name: &str,
    flags: &ScaffoldFlags,
) -> Result<ComposedCombo> {
    let backend = backend_framework(combo).ok_or_else(|| not_a_combo(combo))?;
    // Both halves are validated before a single byte is written: a combo whose
    // server half cannot be produced must not leave a frontend-only directory
    // behind, because that directory looks like a finished project.
    // (Cả hai nửa được kiểm trước khi ghi bất cứ byte nào: combo không tạo được
    // nửa server không được để lại thư mục chỉ có frontend, vì nó trông như
    // project hoàn chỉnh.)
    let half = server_half(backend).ok_or_else(|| no_server_half(combo, backend))?;
    let frontend_framework = frontend_half_framework(backend, combo);
    let initializer = lookup(Mode::Frontend, frontend_framework)
        .ok_or_else(|| no_frontend_half(combo, frontend_framework))?;
    if let Err(refusal) = check(&initializer) {
        anyhow::bail!("refusing initializer: {}", refusal.message());
    }

    validate_project_name(project_name)?;
    let target = Path::new(project_name);
    if target.try_exists()? {
        return Err(crate::error::dir_already_exists(target));
    }

    // The frontend half is written by its own initializer into a staging tree
    // and copied over exactly as the single-framework frontend lane does, so
    // provenance and owned metadata behave identically.
    // (Nửa frontend do initializer của nó ghi vào cây staging rồi chép sang y
    // như lane frontend một framework, để provenance và metadata sở hữu giống
    // hệt.)
    let mut config = crate::wizard::engine::ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec![frontend_framework.to_string()],
        project_name: project_name.to_string(),
        features: web_features(flags),
        template_dir: std::path::PathBuf::new(),
        from_local: flags.source.from_local.clone(),
        no_local: flags.source.no_local,
    };
    let (workspace, source) = run_create_initializer(&mut config, project_name, initializer)
        .await
        .map_err(|error| frontend_half_failed(combo, frontend_framework, error))?;
    let project_dir = crate::scaffold::Scaffolder::scaffold(&config)
        .map_err(|error| frontend_half_failed(combo, frontend_framework, error))?;
    config.template_dir = std::path::PathBuf::new();
    drop(workspace);

    let server_half_result = match half {
        // A Node starter needs its dependency ranges locked before it can start.
        // (Starter Node cần range dependency được lock trước khi chạy được.)
        ServerHalf::Node => {
            super::node_backend::write(&project_dir, backend, frontend_framework, flags.ts)
        }
        ServerHalf::Native => super::native_backend::write(&project_dir, backend, combo),
    };
    if let Err(error) = server_half_result {
        // Never leave a half-composed combo: the frontend half alone would read
        // as a finished project and hide the missing server.
        // (Không để combo ghép dở: chỉ nửa frontend sẽ trông như project hoàn
        // chỉnh và che mất server còn thiếu.)
        std::fs::remove_dir_all(&project_dir).ok();
        return Err(error);
    }

    Ok(ComposedCombo {
        project_dir,
        provenance: vec![
            format!("combo:{combo}#halves={frontend_framework}+{backend}"),
            source,
        ],
        frontend_framework: frontend_framework.to_string(),
    })
}

/// Error for a combo whose server half has no offline generator.
/// Lỗi cho combo không có generator offline cho nửa server.
///
/// The message names both possible sources — a native half MagiCore writes, and
/// the Node starters it can invoke — because the user has to choose which one
/// they wanted.
/// Thông điệp nêu cả hai nguồn khả dĩ — nửa native MagiCore tự ghi, và starter
/// Node MagiCore gọi được — vì người dùng phải chọn điều họ muốn.
fn no_server_half(combo: &str, backend: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "the '{combo}' combo needs both halves and MagiCore has no offline generator for its \
         '{backend}' server half. Scaffold each half with `mgc create-web <frontend> <name>` and \
         `mgc create-web {backend} <name>`, or supply the combo through the registry layer / \
         `--from-local <path>`. Nothing was created."
    )
}

/// Error when the frontend half cannot be produced.
/// Lỗi khi không tạo được nửa frontend.
fn frontend_half_failed(combo: &str, frontend: &str, error: anyhow::Error) -> anyhow::Error {
    anyhow::anyhow!(
        "the '{combo}' combo needs both halves; its '{frontend}' frontend half could not be \
         scaffolded offline: {error}. Scaffold the halves separately with \
         `mgc create-web {frontend} <name>` (with network access) and `mgc create-web \
         {backend} <name>`, or supply the combo through `--from-local <path>`.",
        combo = combo,
        frontend = frontend,
        backend = backend_framework(combo).unwrap_or("<backend>"),
        error = error
    )
}

/// Error for a combo whose frontend framework publishes no initializer.
/// Lỗi cho combo có framework frontend không có initializer.
fn no_frontend_half(combo: &str, frontend: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "the '{combo}' combo needs both halves and MagiCore has no official initializer for its \
         '{frontend}' frontend half. Scaffold each half with `mgc create-web {frontend} <name>` \
         and `mgc create-web {backend} <name>`, or supply the combo through `--from-local <path>`. \
         Nothing was created.",
        combo = combo,
        frontend = frontend,
        backend = backend_framework(combo).unwrap_or("<backend>")
    )
}

/// Error for a combination this composer does not handle.
/// Lỗi cho tổ hợp composer này không xử lý.
///
/// It never substitutes for a missing server half: the caller checks that first,
/// so the user always learns which half is missing.
/// Lỗi này không bao giờ thay cho nửa server còn thiếu: caller đã kiểm trước, nên
/// người dùng luôn biết thiếu nửa nào.
fn not_a_combo(combo: &str) -> anyhow::Error {
    anyhow::anyhow!("'{combo}' is not a split fullstack combo")
}
