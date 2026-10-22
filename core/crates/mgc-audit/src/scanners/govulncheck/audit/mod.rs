//! Go audit dispatch facade.
//! (Facade điều phối audit Go.)

mod audit;
mod osv_fallback;

pub use audit::audit_go;
