//! Behavior tests for scaffold artifact naming.
//! Test hành vi đặt tên artifact scaffold.

use super::super::artifact_name;
use crate::scaffold::spec::CoreKind;

#[test]
fn uses_the_existing_core_and_template_name_format() {
    assert_eq!(
        artifact_name(CoreKind::Web, "nextjs"),
        "mgc-create-web-nextjs"
    );
}
