//! Converts the accepted v4 provenance wire shape into the shared Provenance type.
//! File này chỉ nối và export owner provenance.

mod provenance;

pub(super) use provenance::deserialize_v4_provenance;
