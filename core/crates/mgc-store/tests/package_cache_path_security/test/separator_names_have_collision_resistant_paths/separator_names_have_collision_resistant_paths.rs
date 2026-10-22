// Distinguish names that flattened to one legacy directory — Tách tên từng bị ép về cùng thư mục cũ.
// This covers colon, underscore and scoped slash forms — Bài này kiểm tra dấu hai chấm, gạch dưới và slash scoped.
#[test]
fn separator_names_have_collision_resistant_paths() {
    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let colon = PackageId::parse("group:artifact@1.2.3").unwrap();
    let underscore = PackageId::parse("group_artifact@1.2.3").unwrap();
    let first_scope = PackageId::parse("@a/b_c@1.2.3").unwrap();
    let second_scope = PackageId::parse("@a_b/c@1.2.3").unwrap();

    assert_ne!(cache.tarball_path(&colon), cache.tarball_path(&underscore));
    assert_ne!(
        cache.tarball_path(&first_scope),
        cache.tarball_path(&second_scope)
    );
}
