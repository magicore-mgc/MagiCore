# Generator error owner

This owner keeps graph, identity, lockfile-verification and JSON-serialization failures typed inside the generator subtree, then maps them to the existing public SbomError variants. Display text and source-bearing public variants remain stable. No retry behavior is introduced.
