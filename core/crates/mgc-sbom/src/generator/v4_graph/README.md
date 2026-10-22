# V4 graph owner

Verifies and enforces the existing lock policy before projecting package-key identities and dependency edges. Package source IDs are hashed into references and are never copied to public SBOM fields.

The owner test checks the early schema guard; full policy, graph and secrecy cases remain integration tests.
