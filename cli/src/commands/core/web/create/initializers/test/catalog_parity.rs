//! Registry↔catalog parity: advertised means scaffoldable, or explicitly not.
//! Parity registry↔catalog: được công bố nghĩa là scaffold được, hoặc chưa.
//!
//! This is the drift guard for the over-claim that made `mgc create-web fastify
//! x` advertise support and then fail closed. A backend may be advertised only
//! when it has a real scaffold source — an initializer catalog record, or a
//! framework-MagiCore layer the create lane can materialize — and every id
//! without a source must live in the explicit `WEB_BACKEND_UNSUPPORTED` list
//! with its reason documented there.
//! (Đây là chốt chống lệch cho lỗi over-claim. Backend chỉ được công bố khi có
//! nguồn scaffold thật — record initializer, hoặc layer MagiCore mà create lane
//! materialize được — và mọi id không nguồn phải nằm trong danh sách
//! `WEB_BACKEND_UNSUPPORTED` với lý do ghi tại đó.)

use crate::commands::core::web::implementation::create::initializers::catalog::{
    BACKEND, FRONTEND,
};
use crate::scaffold::framework_registry::{
    WEB_BACKEND, WEB_BACKEND_UNSUPPORTED, is_known_web_unsupported, is_supported,
};

/// Whether `id` exists in the initializer catalog backend table.
/// (Id có trong bảng backend của catalog initializer không.)
fn has_catalog_record(id: &str) -> bool {
    BACKEND.iter().any(|(record_id, _)| *record_id == id)
}

/// Every backend id with an initializer record is advertised.
///
/// A record is the strongest scaffold source; a registry edit that drops the
/// name while the record stays is exactly the drift this guards.
/// (Mọi id backend có record initializer đều được công bố. Record là nguồn
/// mạnh nhất; registry bỏ tên trong khi record còn lại là đúng kiểu lệch cần
/// chặn.)
#[test]
fn every_backend_with_a_record_is_advertised() {
    for (id, _) in BACKEND {
        assert!(
            WEB_BACKEND.contains(id),
            "backend '{id}' has an initializer record but is not advertised; \
             `create-web {id}` would reject a name the product can scaffold"
        );
    }
}

/// Every advertised web frontend must have an initializer record.
///
/// Frontends are the frameworks that DO work; this pins that they keep a
/// record so a registry edit can never silently drop one.
/// (Mọi frontend web công bố phải có record initializer. Đây là frontend đang
/// chạy được; test ghim để một lần sửa registry không lặng lẽ bỏ mất record.)
#[test]
fn every_advertised_frontend_has_a_catalog_record() {
    use crate::scaffold::framework_registry::WEB_FRONTEND;
    for id in WEB_FRONTEND {
        assert!(
            FRONTEND.iter().any(|(record_id, _)| record_id == id),
            "advertised web frontend '{id}' has no initializer record"
        );
    }
}

/// The advertised and unsupported backend sets are disjoint and consistent with
/// the gate: advertised ⇒ accepted, unsupported ⇒ rejected and reported.
/// (Hai tập backend công bố và chưa hỗ trợ rời nhau và nhất quán với cổng.)
#[test]
fn backend_ids_partition_into_supported_and_explicitly_unsupported() {
    for id in WEB_BACKEND {
        assert!(
            is_supported("web", id),
            "advertised backend '{id}' must be accepted by is_supported"
        );
        assert!(
            !is_known_web_unsupported(id),
            "'{id}' is advertised and unsupported at the same time"
        );
        assert!(
            !WEB_BACKEND_UNSUPPORTED.contains(id),
            "'{id}' appears in both backend lists"
        );
    }
    for id in WEB_BACKEND_UNSUPPORTED {
        assert!(
            !is_supported("web", id),
            "unsupported backend '{id}' must NOT be accepted by is_supported"
        );
        assert!(
            is_known_web_unsupported(id),
            "unsupported backend '{id}' must be reported as known-unsupported"
        );
        assert!(
            !has_catalog_record(id),
            "'{id}' has a catalog record and must therefore be advertised, \
             not listed as unsupported"
        );
    }
}

/// The specific regression: a backend with no scaffold source is not advertised
/// and is reported as not-yet-scaffoldable, never as a silent registry miss.
///
/// Each id here is documented in the initializers README as advertised without
/// a usable record; keeping the list explicit means a framework can only
/// graduate by gaining a real source.
/// (Hồi quy cụ thể: backend không có nguồn không được công bố và được báo là
/// chưa scaffold được. Mỗi id đây được README initializers ghi là công bố
/// nhưng chưa có record dùng được.)
#[test]
fn frameworks_without_a_source_are_reported_as_not_yet_supported() {
    for id in ["fastify", "hono", "koa", "spring", "trpc"] {
        assert!(
            WEB_BACKEND_UNSUPPORTED.contains(&id),
            "'{id}' has no scaffold source and must be in WEB_BACKEND_UNSUPPORTED"
        );
        assert!(
            !is_supported("web", id),
            "'{id}' has no scaffold source and must not be advertised"
        );
        assert!(
            is_known_web_unsupported(id),
            "'{id}' must be reported as not yet supported, not unknown"
        );
    }
}

/// No backend name is advertised twice.
/// (Không tên backend nào được công bố hai lần.)
#[test]
fn advertised_backends_have_no_duplicates() {
    let mut seen = std::collections::BTreeSet::new();
    for id in WEB_BACKEND.iter().chain(WEB_BACKEND_UNSUPPORTED.iter()) {
        assert!(seen.insert(*id), "'{id}' appears more than once");
    }
}
