//! Registry URL builders for App ecosystems.
//! Tạo URL registry cho các ecosystem App.
/// Construct URL for Flutter package on pub.dev.
/// Xây dựng URL cho Flutter package trên pub.dev.
pub fn pub_dev_package_url(package_name: &str) -> String {
    format!("https://pub.dev/packages/{}", package_name)
}

/// Construct URL for Maven artifact.
/// Xây dựng URL cho Maven artifact.
pub fn maven_central_url(group_id: &str, artifact_id: &str, version: &str) -> String {
    let group_path = group_id.replace('.', "/");
    format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.jar",
        group_path, artifact_id, version, artifact_id, version
    )
}

/// Construct URL for CocoaPods spec.
/// Xây dựng URL cho CocoaPods spec.
pub fn cocoapods_spec_url(pod_name: &str) -> String {
    format!("https://cdn.cocoapods.org/Specs/{}.podspec.json", pod_name)
}
