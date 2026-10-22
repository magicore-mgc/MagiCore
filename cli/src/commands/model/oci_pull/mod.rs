// Wires the oci_pull owner; behavior lives in oci_pull.rs.
// Nối owner oci_pull; xử lý nằm trong oci_pull.rs.
mod error;
mod oci_pull;
pub(super) use self::error::OciPullError;
pub(super) use self::oci_pull::pull_oci;
