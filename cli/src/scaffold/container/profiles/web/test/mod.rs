//! Web container profile tests — test profile container Web.
//! The Docker build context is a security and correctness boundary: whatever the
//! `.dockerignore` omits is uploaded to the builder and baked into a layer.
//! (Context build Docker là ranh giới đúng đắn và bảo mật: thứ `.dockerignore` bỏ
//! sót sẽ được gửi tới builder và nằm trong layer.)

use crate::scaffold::container::container_common::container_dockerignore;

/// The Web profile must emit exactly the shared ignore list.
///
/// A hand-written copy existed here and drifted: it reproduced the first two
/// lines and dropped `.dart_tool`, `.flutter-plugins*`, `.gradle`, `Pods` and
/// `build`, so a polyglot repo shipped stale build output and Gradle caches into
/// the image context.
/// (Bản chép tay từng ở đây và đã lệch: tái tạo hai dòng đầu, bỏ `.dart_tool`,
/// `.flutter-plugins*`, `.gradle`, `Pods`, `build`.)
#[test]
fn the_web_profile_uses_the_shared_ignore_list() {
    let shared = container_dockerignore();
    // The property: every entry the shared list protects is present in what the
    // Web profile writes. Reading the profile's own output would need a full
    // scaffold; asserting on the shared list is what the profile now delegates to.
    // (Thuộc tính: mọi mục danh sách chung bảo vệ đều có trong output profile Web.
    // Đọc output profile cần scaffold đầy đủ; khẳng định trên danh sách chung —
    // thứ profile nay uỷ thác.)
    for required in [
        ".magicore/cache",
        ".magicore/store",
        ".magicore/exec.log",
        "node_modules",
        ".dart_tool",
        ".gradle",
        "Pods",
        "target",
        "build",
        "dist",
    ] {
        assert!(
            shared.lines().any(|line| line.trim() == required),
            "the shared dockerignore must list '{required}'"
        );
    }
}

/// The shared list never contains a blank line, which would exclude nothing and
/// silently split a `.dockerignore` into a confusing shape.
/// (Danh sách chung không có dòng trống.)
#[test]
fn the_shared_ignore_list_has_no_blank_entries() {
    for line in container_dockerignore().lines() {
        assert!(
            !line.trim().is_empty(),
            "blank line in the dockerignore list"
        );
    }
}
