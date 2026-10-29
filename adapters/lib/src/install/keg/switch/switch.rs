//! Switch the active keg version of a library without deleting any keg.
//! Đổi version keg active của một library mà không xoá keg nào.
//!
//! Order of operations, fail-closed at every step: take the per-library
//! exclusive guard; verify the target keg (immutable and unchanged) and the
//! keg that is currently active so a tampered target is never published;
//! re-expose every existing version prefix to repair stale links; move the
//! active link; write the active pointer last because it is the commit marker.
//! Any failure before that last write restores every captured entry and
//! pointer, so the previous active version stays active.
//! Thứ tự thao tác (fail-closed từng bước): lấy guard, xác minh keg đích và
//! keg đang active, link lại mọi prefix version, đổi link active, ghi con trỏ
//! cuối cùng. Lỗi trước bước 5 khôi phục mọi entry + con trỏ đã chụp.

use semver::Version;

use crate::install::keg::layout::{ACTIVE_LINK, validate_version};

use crate::install::keg::active::{ActiveState, read_active, write_active};
use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::guard::{KegGuard, capture_entry, restore_entry};
use crate::install::keg::keg::KegStore;
use crate::install::keg::prefix::{place_link, plan_links};

/// Outcome of a successful switch.
/// Kết quả của một lần switch thành công.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchOutcome {
    /// Library name.
    /// Tên library.
    pub name: String,
    /// Version that is now active.
    /// Version đang active sau switch.
    pub active: String,
    /// Version that was active before (still installed, still switchable).
    /// Version active trước đó (vẫn cài, vẫn switch lại được).
    pub previous: Option<String>,
    /// Number of prefix/active links (re)placed.
    /// Số link prefix/active đã đặt (lại).
    pub links: usize,
}

/// Make `version` the active version of `name`, keeping every other keg.
/// Đưa `version` thành version active của `name`, giữ mọi keg khác.
pub fn switch_active(store: &KegStore, name: &str, version: &str) -> KegResult<SwitchOutcome> {
    let layout = store.layout();
    let version = validate_version(version)?;

    let before = read_active(layout, name)?;
    let mut guard = KegGuard::acquire(layout, name)?;

    guard.run(|scope| switch_locked(scope, store, name, &version, before.clone()))
}

fn switch_locked(
    scope: &mut KegGuard,
    store: &KegStore,
    name: &str,
    version: &Version,
    before: Option<ActiveState>,
) -> KegResult<SwitchOutcome> {
    let layout = store.layout();

    // 2. Target must be a real, sealed and unchanged keg.
    // (Keg đích phải là keg thật, đã seal và không đổi.)
    let target_record = store.open_keg(name, &version.to_string())?;

    // The previously active keg must still be trustworthy; otherwise refuse to
    // report it as the rollback target without saying so.
    // (Keg active cũ phải còn nguyên; nếu không, báo lỗi thay vì nhận là đích rollback.)
    if let Some(state) = &before
        && let Ok(previous_version) = validate_version(&state.version)
        && previous_version != *version
    {
        {
            {
                store.open_keg(name, &state.version).map_err(|error| {
                    KegError::Tampered {
                        path: layout.keg_dir(name, &previous_version).display().to_string(),
                        detail: format!(
                            "previously active version '{}' cannot serve as rollback target: {error}",
                            state.version
                        ),
                    }
                })?;
            }
        }
    }

    // Undo for the pointer: captured once, restored first on abort.
    // (Hoàn tác con trỏ: chụp một lần, khôi phục đầu tiên khi abort.)
    scope.push_undo(pointer_undo(layout, name, before.clone()));

    // 3. Re-expose every installed keg so its version prefix is repaired, but
    // never move the active link here: plan_links also yields `current`, and
    // moving it per version would leave the last sorted version active.
    // (Link lại mọi keg để sửa prefix, nhưng KHÔNG đụng link active ở đây:
    // plan_links cũng trả `current`; đổi nó theo từng version sẽ để version cuối
    // cùng thành active.)
    let active_link_path = layout.opt_dir(name).join(ACTIVE_LINK);
    let mut links = 0usize;
    for record in store.list_kegs(name)? {
        let installed = validate_version(&record.version)?;
        // Verify instead of trusting the directory name: a tampered keg must
        // never be exposed (or left exposed) through a repaired prefix.
        // (Xác minh thay vì tin tên thư mục: keg bị sửa không được phơi ra qua
        // prefix vừa sửa.)
        store.open_keg(name, &installed.to_string())?;
        let keg_dir = layout.keg_dir(name, &installed);
        let mut planned = plan_links(layout, name, &installed, &keg_dir)?;
        if installed != *version {
            planned.retain(|link| link.path != active_link_path);
        }
        for link in planned {
            let state = capture_entry(&link.path)?;
            scope.push_undo(restore_entry(link.path.clone(), state));
            crate::install::keg::prefix::ensure_link_parents(std::slice::from_ref(&link))?;
            place_link(&link.path, &link.target)?;
            links += 1;
        }
    }

    // 4. Re-verify the target, then move the single active link last before
    // the pointer (5). Both are captured above so a failure restores them.
    // (4. Xác minh lại đích, rồi đổi duy nhất link active trước con trỏ (5).
    // Cả hai đã được chụp ở trên nên lỗi sẽ khôi phục.)
    store.open_keg(name, &target_record.version)?;
    for link in plan_links(layout, name, version, &layout.keg_dir(name, version))?
        .into_iter()
        .filter(|link| link.path == active_link_path)
    {
        place_link(&link.path, &link.target)?;
        links += 1;
    }

    // 5. Pointer last: it is the commit marker for readers.
    // (Con trỏ ghi cuối: là mốc commit cho bên đọc.)
    let previous = before
        .as_ref()
        .map(|state| state.version.clone())
        .filter(|value| value != &target_record.version);
    write_active(
        layout,
        name,
        &ActiveState {
            version: target_record.version.clone(),
            previous: previous.clone(),
        },
    )?;

    // 6. Commit: discard the undo steps registered for this scope.
    // (Commit: bỏ các bước hoàn tác đã đăng ký trong phạm vi.)
    scope.commit();
    Ok(SwitchOutcome {
        name: name.to_string(),
        active: target_record.version,
        previous,
        links,
    })
}

fn pointer_undo(
    layout: &crate::install::keg::layout::KegLayout,
    name: &str,
    before: Option<ActiveState>,
) -> crate::install::keg::guard::Undo {
    let layout = layout.clone();
    let name = name.to_string();
    Box::new(move || match before {
        Some(state) => {
            let _ = write_active(&layout, &name, &state);
        }
        // No pointer existed: remove the one written by the aborted switch.
        // (Trước đó không có con trỏ: xoá con trỏ do switch bị huỷ ghi.)
        None => {
            let _ = std::fs::remove_file(crate::install::keg::active::pointer_path(&layout, &name));
        }
    })
}

/// Restore the version recorded as `previous`; fails when none is recorded.
/// Khôi phục version đã ghi là `previous`; lỗi khi chưa có.
pub fn rollback_active(store: &KegStore, name: &str) -> KegResult<SwitchOutcome> {
    let state = read_active(store.layout(), name)?
        .ok_or_else(|| KegError::NoActiveVersion(name.to_string()))?;
    let previous = state.previous.clone().ok_or_else(|| {
        KegError::NoActiveVersion(format!("{name} (no previous version recorded)"))
    })?;
    switch_active(store, name, &previous)
}

/// Current active state, if any (read-only, no lock).
/// Trạng thái active hiện tại nếu có (chỉ đọc, không khóa).
pub fn active_state(store: &KegStore, name: &str) -> KegResult<Option<ActiveState>> {
    read_active(store.layout(), name)
}
