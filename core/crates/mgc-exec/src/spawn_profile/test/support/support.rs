use crate::policy::LaneProfile;
use crate::spawn_profile::{SPAWN_PROFILE_SCHEMA, SpawnProfileFile};

pub(crate) fn sample() -> SpawnProfileFile {
    SpawnProfileFile {
        schema_version: SPAWN_PROFILE_SCHEMA.to_string(),
        mgc_version: "1.1.0-rc.9".to_string(),
        corpus_id: "corpus-2026-10-07".to_string(),
        lock_digest: Some("blake3-deadbeef".to_string()),
        captured_at: 1_000_000,
        profile: LaneProfile {
            framework: "next".to_string(),
            version: "16.3.8".to_string(),
            os: "darwin".to_string(),
            operation: "dev".to_string(),
            allowed_pm_invocations: vec!["npm --version".to_string()],
            requires_pm: vec![],
        },
    }
}
