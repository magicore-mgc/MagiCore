//! Build and execute one shared native registry protocol.
//! Dựng và chạy protocol registry native dùng chung.

use crate::adapter::types::{LibAdapter, ResolveFirst};
use crate::native::engine::{
    NativeResolution, resolve_with_protocol, resolve_with_protocol_including_dev,
};
use mgc_resolver::protocols::RegistryProtocol;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

use super::metadata::{NativeRegistryMetadata, NativeRegistryOperation, metadata_for_lane};

/// A configured protocol and its lock projection identity.
/// Protocol đã cấu hình cùng identity để chiếu vào lock.
pub(in crate::adapter) struct NativeRegistry {
    protocol: Box<dyn RegistryProtocol>,
    metadata: NativeRegistryMetadata,
}

impl NativeRegistry {
    /// Build the configured protocol for one existing native Lib lane.
    /// Dựng protocol theo cấu hình của một native Lib lane hiện có.
    pub(in crate::adapter) async fn for_lane(
        adapter: &LibAdapter,
        project_root: &Path,
        lane: &ResolveFirst,
        operation: NativeRegistryOperation,
    ) -> MgResult<Self> {
        let metadata = metadata_for_lane(lane, operation);
        let protocol = match lane {
            ResolveFirst::Python => super::python::create(adapter, project_root)?,
            ResolveFirst::Rust => super::rust::create(),
            ResolveFirst::Go => super::go::create(),
            ResolveFirst::DotNet => super::dotnet::create(adapter).await,
            ResolveFirst::JavaPom => super::java::create(),
        };
        Ok(Self { protocol, metadata })
    }

    /// Resolve a manifest through the native engine using this lane's metadata.
    /// Resolve manifest qua engine native với metadata của lane này.
    pub(in crate::adapter) async fn resolve(
        &self,
        manifest: &Manifest,
        include_dev: bool,
    ) -> MgResult<NativeResolution> {
        if include_dev {
            resolve_with_protocol_including_dev(
                self.protocol.as_ref(),
                self.metadata.ecosystem,
                self.metadata.registry,
                manifest,
            )
            .await
        } else {
            resolve_with_protocol(
                self.protocol.as_ref(),
                self.metadata.ecosystem,
                self.metadata.registry,
                manifest,
            )
            .await
        }
    }
}
