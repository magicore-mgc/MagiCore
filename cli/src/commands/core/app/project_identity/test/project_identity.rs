//! App project identity and language selection tests.
//! Test nhận diện project và chọn ngôn ngữ App.

use super::super::language;

#[test]
fn language_detects_flutter_from_regular_pubspec() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("pubspec.yaml"), "name: sample\n").unwrap();

    assert_eq!(
        language(project.path()).unwrap(),
        mgc_app_adapter::AppLanguage::Flutter
    );
}
