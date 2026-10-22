//! Returns the shared Docker build-context exclusions for non-Web profiles.
//! Trả về các mục loại trừ context build Docker dùng chung cho profile không phải Web.

pub(in crate::scaffold::container) fn container_dockerignore() -> &'static str {
    concat!(
        ".git\n.env\n.env.*\n",
        ".magicore/cache\n.magicore/store\n.magicore/exec.log\n",
        "node_modules\n.dart_tool\n.flutter-plugins\n.flutter-plugins-dependencies\n",
        ".gradle\nPods\ntarget\nbuild\ndist\n"
    )
}
