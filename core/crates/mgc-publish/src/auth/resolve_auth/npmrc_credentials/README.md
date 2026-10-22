# npmrc credential source

EN: Resolves the registry-scoped npmrc token first, then the matching Basic credentials.
VI: Phân giải token npmrc theo registry trước, sau đó đến Basic credentials phù hợp.

Tests: test/npmrc_credentials.rs covers the existing token-before-Basic behavior.
