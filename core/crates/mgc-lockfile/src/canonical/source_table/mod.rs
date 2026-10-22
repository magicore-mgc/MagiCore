//! Writes one source table using canonical field and array order.
//! File này chỉ nối và export owner source_table.

mod source_table;

pub(super) use source_table::push_source;
