//! Configure ignore sources — cấu hình nguồn luật loại trừ.

/// Apply npm or git ignore source precedence to the existing walker.
/// Áp dụng precedence npm hoặc git ignore lên walker hiện có.
pub(in crate::ignore) fn configure(builder: &mut ::ignore::WalkBuilder, has_npmignore: bool) {
    if has_npmignore {
        // npm behavior: .npmignore replaces .gitignore — npm: .npmignore thay thế .gitignore.
        builder.git_ignore(false);
        builder.git_global(false);
        builder.git_exclude(false);
        builder.add_custom_ignore_filename(".npmignore");
    } else {
        builder.ignore(true); // Read .gitignore and .ignore — đọc cả .gitignore và .ignore.
        builder.git_ignore(true);
        builder.git_global(true);
        builder.git_exclude(true);
    }
}
