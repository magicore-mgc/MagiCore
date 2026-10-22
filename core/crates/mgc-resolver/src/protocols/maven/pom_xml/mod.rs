//! XML normalization, bounded tag scanning, and block removal.
//! Chuẩn hóa XML, quét tag có giới hạn và loại bỏ khối.

mod pom_xml;
pub use pom_xml::element_blocks;
pub(super) use pom_xml::{normalize_pom_xml, remove_element_blocks};
