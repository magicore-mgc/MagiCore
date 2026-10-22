# Registry authentication

Owns credential parsing, user/auth state, trusted-publishing identities, authorization policy and Axum middleware. The public contract remains `crate::auth::*`; this folder separates existing operations without changing their decisions or status codes.

Module xác thực registry quản lý parsing credential, state user/auth, trusted publishing, authorization và Axum middleware. API public `crate::auth::*` được giữ nguyên; việc tách module không đổi quyết định quyền hay status code.
