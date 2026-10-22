//! Immutable identity and routing metadata for one Web framework.
//! Metadata bất biến về danh tính và tuyến của một framework Web.

pub(in crate::scaffold::processor) struct WebFrameworkConfig {
    pub(in crate::scaffold::processor) name: &'static str,
    pub(in crate::scaffold::processor) sub_type: &'static str,
    pub(in crate::scaffold::processor) base: Option<&'static str>,
}
