use super::super::GameAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;

impl ScaffoldProvider for GameAdapter {
    /// Evidence: src/scaffold (bevy/godot/unity/unreal generators) + the
    /// `mgc create-game` CLI lane.
    /// Dẫn chứng: src/scaffold (bộ sinh bevy/godot/unity/unreal) + lane
    /// CLI `mgc create-game`.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}
