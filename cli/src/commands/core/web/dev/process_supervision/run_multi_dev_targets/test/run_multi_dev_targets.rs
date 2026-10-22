use super::super::run_multi_dev_targets;
use crate::commands::compat::CompatMode;

#[tokio::test]
async fn empty_target_group_is_a_noop() {
    run_multi_dev_targets(&[], &CompatMode::Native, None)
        .await
        .expect("empty target group is a no-op");
}
