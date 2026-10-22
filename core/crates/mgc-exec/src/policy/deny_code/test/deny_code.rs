use crate::policy::DenyCode;

#[test]
fn denial_codes_remain_machine_distinct() {
    assert_ne!(DenyCode::NativeMutationDeny, DenyCode::OutsideActiveProfile);
    assert_ne!(
        DenyCode::ForbiddenNameNoException,
        DenyCode::MissingException
    );
}
