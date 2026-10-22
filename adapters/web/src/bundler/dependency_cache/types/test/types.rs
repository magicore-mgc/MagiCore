//! Public dependency-result shape test.
//! Test hình dạng kết quả dependency public.

use super::CachedDep;

#[test]
fn public_output_fields_remain_constructible_and_cloneable() {
    let output = CachedDep {
        js: "export default 3;".to_string(),
        css: None,
    };
    let cloned = output.clone();

    assert_eq!(cloned.js, "export default 3;");
    assert_eq!(cloned.css, None);
}
