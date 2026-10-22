// Public plugin model and adapter-backed constructors.
// Model plugin công khai và constructor dựa trên adapter.
use super::adapter_bridge::{
    CacheUnsupported, FetcherBridge, LinkerBridge, ResolverBridge, TemplateUnsupported,
};
use crate::{CacheBackend, Fetcher, Linker, Resolver, TemplateGenerator};
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;
use std::sync::Arc;

/// Plugin combines five ecosystem capabilities, with defaults for unoverridden traits.
/// Plugin ghép năm capability của ecosystem, dùng mặc định cho trait chưa override.
#[derive(Clone)]
pub struct Plugin {
    pub ecosystem: Ecosystem,
    pub resolver: Arc<dyn Resolver>,
    pub fetcher: Arc<dyn Fetcher>,
    pub linker: Arc<dyn Linker>,
    pub template: Arc<dyn TemplateGenerator>,
    pub cache: Arc<dyn CacheBackend>,
    adapter: Option<Arc<dyn PackageAdapter>>,
    // Built-in adapter configuration belongs to one project context.
    // Adapter native đã gắn context không được tái dùng cho project khác.
    context_scoped: bool,
}

impl Plugin {
    /// Builds a plugin from the legacy adapter and forwards each capability.
    /// Tạo plugin từ adapter cũ và chuyển tiếp từng capability tương ứng.
    pub fn from_adapter(adapter: Arc<dyn PackageAdapter>) -> Self {
        let resolver = Arc::new(ResolverBridge(adapter.clone()));
        let fetcher = Arc::new(FetcherBridge(adapter.clone()));
        let linker = Arc::new(LinkerBridge(adapter.clone()));
        Self {
            ecosystem: adapter.ecosystem(),
            resolver,
            fetcher,
            linker,
            template: Arc::new(TemplateUnsupported {
                ecosystem: adapter.ecosystem(),
            }),
            cache: Arc::new(CacheUnsupported {
                ecosystem: adapter.ecosystem(),
            }),
            adapter: Some(adapter),
            context_scoped: false,
        }
    }

    /// Register a native bridge without making its adapter a global context cache.
    /// Giữ bridge trong registry; mỗi project phải tạo lại adapter theo cấu hình riêng.
    pub fn from_project_adapter(adapter: Arc<dyn PackageAdapter>) -> Self {
        let mut plugin = Self::from_adapter(adapter);
        plugin.context_scoped = true;
        plugin
    }

    /// Whether factory callers must bind a fresh adapter to their own project.
    /// Không tái dùng back-reference chứa registry/ngôn ngữ của context khác.
    pub fn is_context_scoped(&self) -> bool {
        self.context_scoped
    }

    /// Returns the adapter ecosystem for registry-miss fallback without re-registering.
    /// Trả ecosystem của adapter để fallback khi registry miss, không đăng ký lại.
    pub fn ecosystem(&self) -> Ecosystem {
        self.ecosystem
    }

    /// Returns the original adapter; native five-trait plugins have no back-reference.
    /// Trả adapter gốc; plugin native chỉ dùng năm trait sẽ không có back-reference.
    pub fn as_adapter(&self) -> Option<Arc<dyn PackageAdapter>> {
        self.adapter.clone()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
