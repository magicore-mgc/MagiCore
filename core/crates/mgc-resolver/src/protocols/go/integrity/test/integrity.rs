use super::super::*;

#[test]
fn dirhash_matches_known_shape() {
    // Two files → sorted lines hashed; stable across call order.
    // (Hai file → hash các dòng đã sắp xếp; ổn định bất kể thứ tự gọi.)
    let mk = |name: &str, data: &[u8]| zip_reader::ZipEntry {
        name: name.to_string(),
        data: data.to_vec(),
            unix_mode: None,
    };
    let a = vec![mk("m@v1/b", b"bbb"), mk("m@v1/a", b"aaa")];
    let b = vec![mk("m@v1/a", b"aaa"), mk("m@v1/b", b"bbb")];
    assert_eq!(
        dirhash_zip(&a).expect("test operation must succeed"),
        dirhash_zip(&b).expect("test operation must succeed")
    );
    assert!(
        dirhash_zip(&a)
            .expect("test operation must succeed")
            .starts_with("h1:")
    );
}
