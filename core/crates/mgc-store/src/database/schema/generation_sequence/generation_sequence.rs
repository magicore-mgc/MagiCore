use anyhow::Result;
use rusqlite::Transaction;

pub(in crate::database::database::schema) fn bootstrap_generation_sequence(
    tx: &Transaction,
) -> Result<()> {
    tx.execute(
        "INSERT INTO cas_generation_seq (project_root, last_issued)
             SELECT project_root, MAX(generation) FROM cas_generations
             GROUP BY project_root
             ON CONFLICT(project_root) DO UPDATE
             SET last_issued = MAX(last_issued, excluded.last_issued)",
        [],
    )?;
    Ok(())
}
