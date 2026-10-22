use super::{add_lease_column, lease_columns_present};
use anyhow::Result;
use rusqlite::Connection;

/// Crash-recovery lease columns (Gate 11-B, vòng-11 verdict): the
/// doctor's stale-staging GC must distinguish "an install is
/// RUNNING (pid alive, lease fresh)" from "the install DIED (pid
/// gone or lease ancient)" — age alone cannot. SQLite has no
/// IF NOT EXISTS for ALTER ADD COLUMN; probe table_info first
/// (idempotent across re-opens, race-free under the schema batch).
/// (Cột lease phục hồi crash: GC stale-staging của doctor phải
/// phân biệt "install đang CHẠY (pid sống, lease mới)" với
/// "install đã CHẾT (pid mất hoặc lease quá cũ)" — chỉ tuổi
/// không đủ. SQLite không có IF NOT EXISTS cho ALTER ADD COLUMN;
/// thăm dò table_info trước (idempotent qua các lần mở lại, không
/// race dưới batch schema).)
pub(in crate::database::database::schema) fn ensure_lease_columns_in_transaction(
    tx: &Connection,
) -> Result<()> {
    // P0-3 (Gate 11-B adversarial review 2026-09-15): the OLD code
    // swallowed EVERY ALTER TABLE error (`let _ = ...`) — a locked,
    // corrupt, or disk-full store silently opened with a MISSING
    // lease schema. Now: only the "duplicate column name" race is
    // tolerated; any other error propagates, and after migrating we
    // REVALIDATE (fail-closed) that both columns are present.
    // (P0-3: code CŨ nuốt MỌI lỗi ALTER TABLE (`let _ = ...`) —
    // store bị khóa/hỏng/đầy đĩa mở im lặng với schema lease THIẾU.
    // Giờ: chỉ race "duplicate column name" được dung thứ; lỗi khác
    // propagate, và sau migrate REVALIDATE (fail-closed) rằng cả
    // hai cột đã hiện diện.)
    let (lease_pid_present, lease_started_present) = lease_columns_present(tx)?;
    if !lease_pid_present {
        add_lease_column(
            tx,
            "lease_pid",
            "ALTER TABLE cas_generations ADD COLUMN lease_pid INTEGER",
        )?;
    }
    if !lease_started_present {
        add_lease_column(
            tx,
            "lease_started_at",
            "ALTER TABLE cas_generations ADD COLUMN lease_started_at INTEGER",
        )?;
    }
    // REVALIDATE (fail-closed): a swallowed/concurrent ALTER must
    // never hand back a DB missing its lease schema — re-probe and
    // error if either column is still absent.
    // (REVALIDATE (fail-closed): ALTER bị nuốt/song song không bao
    // giờ được trả DB thiếu schema lease — thăm dò lại và lỗi nếu
    // cột nào vẫn thiếu.)
    let (lease_pid_present, lease_started_present) = lease_columns_present(tx)?;
    if !lease_pid_present || !lease_started_present {
        return Err(anyhow::anyhow!(
            "store.db schema migration failed: lease columns missing after ALTER \
                     (lease_pid={lease_pid_present}, lease_started_at={lease_started_present})"
        ));
    }
    Ok(())
}

pub(in crate::database::database::schema) fn ensure_lease_columns(
    conn: &mut Connection,
) -> Result<()> {
    let (pid_present, started_present) = lease_columns_present(conn)?;
    if pid_present && started_present {
        return Ok(());
    }

    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    // Re-read after taking the schema write lock: another process may have
    // completed the migration between the optimistic probe and this lock.
    // (Đọc lại sau khi lấy schema write-lock vì process khác có thể migrate
    // xong giữa lần thăm dò ban đầu và lúc lấy khóa.)
    let (pid_present, started_present) = lease_columns_present(&tx)?;
    if !pid_present {
        add_lease_column(
            &tx,
            "lease_pid",
            "ALTER TABLE cas_generations ADD COLUMN lease_pid INTEGER",
        )?;
    }
    if !started_present {
        add_lease_column(
            &tx,
            "lease_started_at",
            "ALTER TABLE cas_generations ADD COLUMN lease_started_at INTEGER",
        )?;
    }
    let (pid_present, started_present) = lease_columns_present(&tx)?;
    if !pid_present || !started_present {
        anyhow::bail!(
            "store.db schema migration failed: lease columns missing after ALTER (lease_pid={pid_present}, lease_started_at={started_present})"
        );
    }
    tx.commit()?;
    Ok(())
}
