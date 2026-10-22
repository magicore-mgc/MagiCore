use super::super::{errors::CasGenerationError, types::Database};
impl Database {
    /// Register that `project_root` references a CAS blob — idempotent per
    /// (project, generation, hash). The claim lands EXACTLY in the
    /// generation the caller's install token names (P0-A token protocol):
    /// a hash already claimed by a previous generation gets a SECOND row in
    /// this generation, so promote of THIS generation keeps protecting it —
    /// the v1 `INSERT OR IGNORE` against PK (project, hash) silently kept
    /// the old-generation row and promote deleted the claim of a blob the
    /// new install still needed (sequential reinstall, no concurrency).
    /// (Đăng ký project tham chiếu blob CAS — idempotent theo (project,
    /// generation, hash). Claim rơi ĐÚNG vào generation mà token install
    /// của caller chỉ định: hash đã được generation trước claim sẽ có row
    /// THỨ HAI ở generation này, nên promote của generation NÀY tiếp tục
    /// bảo vệ nó — `INSERT OR IGNORE` v1 với PK (project, hash) giữ nguyên
    /// row generation cũ và promote xóa claim của blob mà install mới vẫn
    /// cần (reinstall tuần tự, không cần concurrency).)
    pub fn cas_claim(
        &self,
        project_root: &str,
        generation: i64,
        hash: &str,
    ) -> std::result::Result<(), CasGenerationError> {
        self.cas_claim_batch(project_root, generation, &[hash])
    }

    /// Validate a staging token and register a set of CAS claims atomically.
    /// This batches a tarball's file claims into one SQLite writer transaction,
    /// avoiding one connection-lock cycle per file while preserving all-or-none
    /// token validation.
    /// (Kiểm tra token staging và ghi một nhóm CAS claim nguyên tử. Gom claim
    /// từng tarball vào một transaction SQLite, tránh lấy/nhả khóa mỗi file.)
    pub fn cas_claim_batch(
        &self,
        project_root: &str,
        generation: i64,
        hashes: &[&str],
    ) -> std::result::Result<(), CasGenerationError> {
        // SQLite serializes writers for a database file. Multiple package
        // extraction workers run in this process, so queue them explicitly
        // by canonical DB path instead of making independent connections
        // race through SQLite's busy handler. Other projects have a distinct
        // gate; other processes remain protected by SQLite's own locking and
        // busy timeout.
        // (SQLite chỉ cho một writer trên mỗi file DB. Các worker giải nén
        // chạy trong cùng process nên xếp hàng theo canonical DB path thay vì
        // để nhiều connection đua vào busy handler. Project khác có gate riêng;
        // process khác vẫn được bảo vệ bởi khóa SQLite và busy timeout.)
        let _claim_writer = self
            .claim_write_gate
            .lock()
            .map_err(|_| CasGenerationError::Io("local claim-writer gate was poisoned".into()))?;

        self.conn
            .execute_batch("BEGIN IMMEDIATE")
            .map_err(|e| CasGenerationError::Io(format!("begin claim batch transaction: {e}")))?;
        let result = (|| {
            self.validate_cas_claim_staging_token(project_root, generation)?;
            self.insert_cas_claim_rows(project_root, generation, hashes)?;
            Ok(())
        })();

        match result {
            Ok(()) => match self.conn.execute_batch("COMMIT") {
                Ok(()) => Ok(()),
                Err(commit_error) => {
                    let rollback_error = self.conn.execute_batch("ROLLBACK").err();
                    let message = match rollback_error {
                        Some(rollback_error) => format!(
                            "commit claim batch: {commit_error}; rollback also failed: {rollback_error}"
                        ),
                        None => format!("commit claim batch: {commit_error}"),
                    };
                    Err(CasGenerationError::Io(message))
                }
            },
            Err(error) => match self.conn.execute_batch("ROLLBACK") {
                Ok(()) => Err(error),
                Err(rollback_error) => Err(CasGenerationError::Io(format!(
                    "{error}; rollback claim batch also failed: {rollback_error}"
                ))),
            },
        }
    }
}
