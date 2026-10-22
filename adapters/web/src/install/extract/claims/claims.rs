//! Open the fail-closed claim database and expose its transaction tuple.
//! Mở DB claim fail-closed và cung cấp tuple transaction.
use super::CasClaimContext;
use mgc_store::{Database, Layout};
use mgc_types::{MgError, MgResult};

/// Fail-closed claim context (Gate 11-A, P0-5): opening the claim DB is a
/// HARD prerequisite of a claiming install — the error propagates and the
/// install fails, exactly like the primary DB open. There is no code path
/// where blobs materialize unclaimed while the install still promotes.
/// (Context claim fail-closed (P0-5): mở DB claim là điều kiện TIÊN QUYẾT
/// cứng của install có claim — lỗi propagate và install fail, giống hệt mở
/// DB chính. Không tồn tại đường mà blob materialize không claim trong khi
/// install vẫn promote.)
pub fn cas_claim_context(layout: &Layout, generation: i64) -> MgResult<CasClaimContext> {
    let db = Database::open(&layout.db_path()).map_err(|e| {
        MgError::Store(format!(
            "claim database open failed (install refuses to materialize \
             unclaimed blobs — fail-closed, P0-5): {e}"
        ))
    })?;
    Ok(CasClaimContext {
        db,
        project_key: layout.root().to_string_lossy().into_owned(),
        generation,
    })
}

pub fn claim_ctx(ctx: &CasClaimContext) -> (&Database, &str, i64) {
    (&ctx.db, &ctx.project_key, ctx.generation)
}
