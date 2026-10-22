//! Builds the shared Compose document for MGC-backed container profiles.
//! Tạo tài liệu Compose dùng chung cho profile container do MGC quản lý.

use crate::scaffold::container::{DEFAULT_CONTAINER_DATA_DIR, DEFAULT_CONTAINER_DATA_VOLUME};

pub(in crate::scaffold::container) fn container_compose(
    image: &str,
    mgc_image: &str,
    command: &str,
) -> String {
    format!(
        "services:\n\
         \x20\x20app:\n\
         \x20\x20\x20\x20build:\n\
         \x20\x20\x20\x20\x20\x20context: .\n\
         \x20\x20\x20\x20\x20\x20args:\n\
         \x20\x20\x20\x20\x20\x20\x20\x20MGC_IMAGE: ${{MGC_IMAGE:-{mgc_image}}}\n\
         \x20\x20\x20\x20image: ${{MGC_IMAGE_NAME:-{image}}}\n\
         \x20\x20\x20\x20init: true\n\
         \x20\x20\x20\x20command: [\"{command}\"]\n\
         \x20\x20\x20\x20volumes:\n\
         \x20\x20\x20\x20\x20\x20- mgc-data:{DEFAULT_CONTAINER_DATA_DIR}\n\
         volumes:\n\
         \x20\x20mgc-data:\n\
         \x20\x20\x20\x20name: ${{MGC_DATA_VOLUME:-{DEFAULT_CONTAINER_DATA_VOLUME}}}\n"
    )
}
