//! CycloneDX 1.5 output facade with stable public model paths.
//! Facade output CycloneDX 1.5, giữ nguyên đường dẫn model public.

mod cyclonedx;
mod purl;
mod serial;
mod severity;
mod types;

pub use self::cyclonedx::to_cyclonedx;
pub use self::types::{
    CYCLONEDX_BOM_FORMAT, CYCLONEDX_SPEC_VERSION, CycloneDxAffect, CycloneDxBom,
    CycloneDxComponent, CycloneDxMetadata, CycloneDxProperty, CycloneDxRating, CycloneDxSource,
    CycloneDxTool, CycloneDxTools, CycloneDxVulnerability,
};
