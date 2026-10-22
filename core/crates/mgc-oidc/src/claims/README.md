# OIDC claims

English: This facade owns the stable `mgc_oidc::claims::{Audience, OidcClaims}` paths and re-exports the root `mgc_oidc::OidcClaims` type through the existing crate facade.

Tiếng Việt: Facade này giữ các đường dẫn ổn định `mgc_oidc::claims::{Audience, OidcClaims}` và re-export `mgc_oidc::OidcClaims` qua crate facade hiện hữu.

The two data owners remain independent: `Audience` owns the untagged string-or-list wire form; `OidcClaims` owns the verified identity fields and cross-audience workload comparison. No dummy error module is added to these serde-only type owners.
