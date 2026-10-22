# Lockfile provider / Provider lockfile

Owns the lockfile capability probe and manifest writer bridge.

Sở hữu probe lockfile và cầu nối ghi manifest. Lock encoding, graph construction and integrity remain under `crate::lockfile`; no lock behavior moves or changes.
