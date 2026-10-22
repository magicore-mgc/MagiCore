# Launcher policy facade tests / Test facade launcher policy

Keeps construction defaults at the public facade boundary: both `mgc dev` and `mgc test` policies default to disallowing dangerous Deno permissions. Validation behavior itself is tested next to its owner.
