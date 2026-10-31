//! Serializes and parses complete v4 lock documents, then validates revision rules.
//! Serialize và parse toàn bộ lock document v4 rồi kiểm tra revision.

use crate::canonical::{
    LockfilePayload, LockfileV4, PackageV4, edge_table::push_edge, package_key_table,
    toml_escaping::escape_toml_string, toml_fields::push_str_field, validate_v4_revision,
};

/// Serialize the FULL v4 document (payload + hash + signature + timestamps).
///
/// Byte layout rules (the parser binds a bare scalar to whichever
/// `[table]`/`[[array]]` was opened last):
/// 1. every top-level array is declared BEFORE the first header — an
///    empty array declares itself and closes immediately;
/// 2. inside `[[sources]]`/`[[package]]` items, scalars come first and
///    nested tables close the item.
///
/// Every collection is emitted in the canonical total order (name then
/// version, features sorted), so the byte output is a pure function of
/// the document value: two runs over the same graph — whatever order the
/// graph arrived in — produce identical bytes, and the emitted
/// `lockfile_hash` can be recomputed from the file itself.
/// (Serialize TOÀN BỘ tài liệu v4. Mọi collection phát theo thứ tự
/// canonical nên byte là hàm thuần của giá trị tài liệu.)
pub fn write_v4_document(lock: &LockfileV4) -> crate::LockfileResult<String> {
    let payload = lock.payload();
    let mut out = String::new();
    push_root_fields(&mut out, &payload);
    push_sources(&mut out, &payload);
    push_metadata(&mut out, lock);
    push_peer_contexts(&mut out, &payload);
    push_workspace(&mut out, &payload);
    push_packages(&mut out, &payload);
    Ok(out)
}

/// Every root-level value, emitted BEFORE the first header: a bare
/// scalar/array after a `[table]` header would bind to that table.
/// Mọi giá trị cấp root, phát TRƯỚC header đầu tiên: scalar/mảng đứng sau
/// header `[table]` sẽ bị gắn vào bảng đó.
fn push_root_fields(out: &mut String, payload: &LockfilePayload) {
    push_str_field(out, "version", &payload.version);
    if let Some(profile) = &payload.optimizer_profile {
        push_str_field(out, "optimizer_profile", profile);
    }
    if !payload.root_dependencies.is_empty() {
        let mut roots = payload.root_dependencies.clone();
        roots.sort();
        push_string_array(out, "root_dependencies", &roots);
    }
    if !payload.root_dependencies_by_owner.is_empty() {
        // One inline table: repeating the dotted key per owner would emit a
        // duplicate key. BTreeMap iteration already fixes owner order.
        // (Một bảng inline: lặp key dotted theo từng owner sẽ trùng key.
        // Thứ tự owner đã cố định nhờ BTreeMap.)
        let mut owners: Vec<String> = Vec::with_capacity(payload.root_dependencies_by_owner.len());
        for (owner, roots) in &payload.root_dependencies_by_owner {
            let mut roots = roots.clone();
            roots.sort();
            roots.dedup();
            let rendered: Vec<String> = roots.iter().map(|root| escape_toml_string(root)).collect();
            owners.push(format!(
                "{} = [{}]",
                escape_toml_string(owner),
                rendered.join(", ")
            ));
        }
        out.push_str("root_dependencies_by_owner = { ");
        out.push_str(&owners.join(", "));
        out.push_str(" }\n");
    }
    out.push('\n');
}

/// One `[[sources]]` item per declared source, in declaration order.
///
/// Source order is part of precedence (lowest `priority` wins among equal
/// claims) and the reader keeps it, so sorting here would change a document
/// that is otherwise identical.
/// (Thứ tự source là một phần của độ ưu tiên và reader giữ nguyên nên sắp
/// xếp ở đây sẽ làm lệch tài liệu vốn giống hệt.)
fn push_sources(out: &mut String, payload: &LockfilePayload) {
    for source in &payload.sources {
        out.push_str("[[sources]]\n");
        push_str_field(out, "id", &source.id);
        push_str_field(out, "url", &source.url);
        push_str_field(out, "ecosystem", source.ecosystem.as_str());
        out.push_str(&format!("priority = {}\n", source.priority));
        push_string_array(out, "claims", &source.claims);
        out.push_str(&format!("trusted = {}\n", source.trusted));
        push_string_array(out, "allow_hosts", &source.allow_hosts);
        push_string_array(out, "allow_cidrs", &source.allow_cidrs);
        push_string_array(out, "allow_protocols", &source.allow_protocols);
        out.push('\n');
    }
}

/// Peer-context digests and their sorted peer sets.
/// Digest peer-context và tập peer đã sắp xếp.
fn push_peer_contexts(out: &mut String, payload: &LockfilePayload) {
    if payload.peer_contexts.is_empty() {
        return;
    }
    out.push_str("[peer_contexts]\n");
    for (digest, peers) in &payload.peer_contexts {
        let mut sorted = peers.clone();
        sorted.sort();
        let rendered: Vec<String> = sorted
            .iter()
            .map(|(name, version)| {
                format!(
                    "[{}, {}]",
                    escape_toml_string(name),
                    escape_toml_string(version)
                )
            })
            .collect();
        out.push_str(&escape_toml_string(digest));
        out.push_str(" = [");
        out.push_str(&rendered.join(", "));
        out.push_str("]\n");
    }
    out.push('\n');
}

/// Workspace membership and cross-core edges.
/// Thành viên workspace và cạnh liên core.
fn push_workspace(out: &mut String, payload: &LockfilePayload) {
    let Some(workspace) = &payload.workspace else {
        return;
    };
    out.push_str("[workspace]\n");
    let mut members = workspace.members.clone();
    members.sort();
    push_string_array(out, "members", &members);
    let mut cross_core_edges = workspace.cross_core_edges.clone();
    cross_core_edges.sort_by(|left, right| {
        (&left.from_core, &left.to_core, &left.package).cmp(&(
            &right.from_core,
            &right.to_core,
            &right.package,
        ))
    });
    for edge in &cross_core_edges {
        out.push_str("[[workspace.cross_core_edges]]\n");
        push_str_field(out, "from_core", &edge.from_core);
        push_str_field(out, "to_core", &edge.to_core);
        push_str_field(out, "package", &edge.package);
        out.push('\n');
    }
    out.push('\n');
}

/// `[[package]]` items sorted by key, each with sorted edges.
/// Item `[[package]]` sắp theo key, cạnh trong mỗi item đã sắp xếp.
fn push_packages(out: &mut String, payload: &LockfilePayload) {
    let mut packages: Vec<&PackageV4> = payload.packages.iter().collect();
    packages.sort_by_cached_key(|package| crate::canonical::sort_key::package_sort_key(package));
    if packages.is_empty() {
        return;
    }

    for package in packages {
        push_package(out, package);
    }
}

/// One `[[package]]` item: scalars/markers first, target key inline, then
/// nested artifact/provenance tables that close the item.
/// Một item `[[package]]`: scalar/marker trước, key inline, rồi bảng
/// artifact/provenance lồng nhau đóng item.
fn push_package(out: &mut String, package: &PackageV4) {
    out.push_str("[[package]]\n");
    // Scalars first, then the nested key/artifact/provenance tables: each
    // nested table must close the row, or later scalars would land inside.
    // (Scalar trước, rồi bảng key/artifact/provenance lồng nhau: mỗi bảng
    // lồng phải đóng row, nếu không scalar sau sẽ lọt vào trong.)
    if let Some(markers) = &package.markers {
        let mut markers = markers.clone();
        markers.sort();
        push_string_array(out, "markers", &markers);
    }
    if let Some(toolchain) = &package.toolchain {
        push_str_field(out, "toolchain", toolchain);
    }
    if let Some(policy) = &package.scripts_policy {
        push_str_field(out, "scripts_policy", policy);
    }
    if let Some(store_ref) = &package.store_ref {
        push_str_field(out, "store_ref", store_ref);
    }
    // ONE identity writer: package rows and their variants serialize through
    // the same code path the canonical payload uses, so file bytes and
    // digest bytes cannot drift apart.
    // (Một writer danh tính duy nhất: row package và variant đi qua đúng
    // đường mà payload canonical dùng, nên byte file và byte digest không
    // thể lệch.)
    package_key_table(out, &package.key);
    let mut edges = package.edges.clone();
    edges.sort_by_cached_key(crate::canonical::sort_key::edge_sort_key);
    for edge in &edges {
        push_edge(out, edge);
    }
    if let Some(artifact) = &package.artifact {
        out.push_str("[package.artifact]\n");
        push_str_field(out, "url", &artifact.url);
        if let Some(size) = artifact.size_bytes {
            out.push_str(&format!("size_bytes = {size}\n"));
        }
        push_str_field(out, "content_hash", &artifact.content_hash);
        push_str_field(out, "downloaded_from", &artifact.downloaded_from);
        // Only present when a lane supplied registry SRI — documents
        // without it digest exactly as before (no silent digest change).
        // Chỉ có khi lane cung cấp SRI — tài liệu thiếu field digest
        // y hệt như trước (không đổi digest ngầm).
        if let Some(sri) = artifact.integrity_sri.as_deref() {
            push_str_field(out, "integrity_sri", sri);
        }
    }
    if let Some(provenance) = &package.provenance {
        out.push_str("[package.provenance]\n");
        push_str_field(out, "source_kind", &provenance.source_kind);
        if let Some(tool) = &provenance.tool {
            push_str_field(out, "tool", tool);
        }
        if let Some(imported_from) = &provenance.imported_from {
            push_str_field(out, "imported_from", imported_from);
        }
    }
    out.push('\n');
}

/// Out-of-payload metadata: timestamp, digest and optional inline signature.
/// Metadata ngoài payload: timestamp, digest và chữ ký nhúng tùy chọn.
fn push_metadata(out: &mut String, lock: &LockfileV4) {
    out.push_str("[metadata]\n");
    push_str_field(out, "generated_at", &lock.metadata.generated_at);
    push_str_field(out, "generator", &lock.metadata.generator);
    push_str_field(out, "lockfile_hash", &lock.metadata.lockfile_hash);
    out.push('\n');
    // The inline signature authenticates the payload digest written above.
    // (Chữ ký nhúng xác thực digest payload đã ghi ở trên.)
    if let Some(signature) = &lock.metadata.signature {
        out.push_str("[metadata.signature]\n");
        push_str_field(out, "algorithm", &signature.algorithm);
        push_str_field(out, "key_id", &signature.key_id);
        push_str_field(out, "public_key", &signature.public_key);
        push_str_field(out, "digest", &signature.digest);
        push_str_field(out, "signed_at", &signature.signed_at);
        push_str_field(out, "signature", &signature.signature);
        out.push('\n');
    }
}

/// Emit a string array field `key = ["a", "b"]`.
/// Ghi field mảng chuỗi `key = ["a", "b"]`.
fn push_string_array(out: &mut String, key: &str, values: &[String]) {
    let rendered: Vec<String> = values
        .iter()
        .map(|value| escape_toml_string(value))
        .collect();
    out.push_str(key);
    out.push_str(" = [");
    out.push_str(&rendered.join(", "));
    out.push_str("]\n");
}

/// Parse a full v4 document (any field order), rejecting non-v4 versions.
/// (Parse toàn bộ tài liệu v4, từ chối version khác.)
pub fn parse_v4_document(text: &str) -> crate::LockfileResult<LockfileV4> {
    let lock: LockfileV4 = toml::from_str(text)?;
    validate_v4_revision(&lock)?;
    Ok(lock)
}
