//! OCI blob operation wiring that routes each operation to its owner.
//! Wiring OCI blob để chuyển từng thao tác tới owner tương ứng.

mod delete_oci_blob;
mod error;
mod get_oci_blob;
mod mount_oci_blob;
mod oci_blob_exists;
mod put_oci_blob;
