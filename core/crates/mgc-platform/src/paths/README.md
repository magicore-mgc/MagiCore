# Platform paths

Owns global/project data roots and project-root discovery. Shell PATH persistence is delegated to `shell/`; the old `mgc_platform::paths` API remains compatible through re-exports. Global data-root semantics were not changed by this extraction.
