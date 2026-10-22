//! Owns the trusted-publishing router, request DTOs, handlers, access checks, and HTTP response mapping.
//! Sở hữu router trusted publishing, request DTO, handler, access check và ánh xạ HTTP response.

use super::super::package_scope::validate_repository;
use super::super::prelude::*;
use super::super::*;

const MAX_TRUSTED_REQUEST_BYTES: usize = 20 * 1024;

#[derive(Deserialize)]
struct TokenExchangeRequest {
    package: String,
    #[serde(default = "default_protocol")]
    protocol: String,
    oidc_token: String,
    sigstore_oidc_token: String,
}

#[derive(Serialize)]
struct TokenExchangeResponse {
    token: String,
    expires_in: u64,
    token_type: &'static str,
}

#[derive(Deserialize)]
struct BindingRequest {
    package: String,
    #[serde(default = "default_protocol")]
    protocol: String,
    repository: String,
}

#[derive(Deserialize)]
struct SignatureQuery {
    package: String,
    #[serde(default = "default_protocol")]
    protocol: String,
    #[serde(default)]
    after_sequence: u64,
    #[serde(default)]
    through_sequence: Option<u64>,
    #[serde(default)]
    limit: Option<usize>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/-/v1/trusted-publish/token", post(exchange_token))
        .route("/-/v1/trusted-publish/bindings", put(set_binding))
        .route("/-/v1/trusted-publish/signatures", get(get_signatures))
        .layer(DefaultBodyLimit::max(MAX_TRUSTED_REQUEST_BYTES))
}

async fn exchange_token(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Json(request): Json<TokenExchangeRequest>,
) -> Result<(axum::http::HeaderMap, Json<TokenExchangeResponse>), StatusCode> {
    if !auth.trusted_enabled() {
        return Err(StatusCode::NOT_FOUND);
    }
    if !auth.trusted_transport_allowed(&headers, crate::peer_address(peer)) {
        return Err(StatusCode::FORBIDDEN);
    }
    let package = scoped_package(&request.protocol, &request.package)?;
    let (token, expires_in, claims) = auth
        .exchange_trusted_token(&package, &request.oidc_token, &request.sigstore_oidc_token)
        .await?;
    if store
        .audit(
            "trusted-publish-token-issued",
            &package,
            None,
            Some(&claims.sub),
        )
        .await
        .is_err()
    {
        auth.discard_trusted_token(&token);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("no-store"),
    );
    Ok((
        headers,
        Json(TokenExchangeResponse {
            token,
            expires_in,
            token_type: "Bearer",
        }),
    ))
}

async fn set_binding(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Json(request): Json<BindingRequest>,
) -> Result<StatusCode, StatusCode> {
    if !auth.trusted_enabled() {
        return Err(StatusCode::NOT_FOUND);
    }
    if !auth.trusted_transport_allowed(&headers, crate::peer_address(peer)) {
        return Err(StatusCode::FORBIDDEN);
    }
    auth.authorize_admin_async(&headers).await?;
    let package = scoped_package(&request.protocol, &request.package)?;
    validate_repository(&request.repository)?;
    store
        .set_trusted_publisher(&package, &request.repository)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_signatures(
    State((store, auth)): State<AppState>,
    headers: HeaderMap,
    peer: crate::OptionalPeerAddress,
    Query(query): Query<SignatureQuery>,
) -> Result<Json<AttestationBundle>, StatusCode> {
    if !auth.trusted_enabled() {
        return Err(StatusCode::NOT_FOUND);
    }
    let package = scoped_package(&query.protocol, &query.package)?;
    if auth.admin_token.is_some() {
        if !auth.trusted_transport_allowed(&headers, crate::peer_address(peer)) {
            return Err(StatusCode::FORBIDDEN);
        }
        let user = auth
            .authenticate_headers_async(&headers)
            .await
            .ok_or(StatusCode::UNAUTHORIZED)?;
        if !auth.can_access(&user, &package) {
            return Err(StatusCode::FORBIDDEN);
        }
    }
    let limit = query.limit.unwrap_or(SIGNATURE_PAGE_SIZE);
    if limit == 0 || limit > MAX_SIGNATURE_PAGE_SIZE {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (head_sequence, entries) = store
        .trusted_attestations_page(
            &package,
            query.after_sequence,
            query.through_sequence,
            limit,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let key = auth.attestation_key().ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(AttestationBundle {
        package,
        public_key: key.public_key.to_base64(),
        key_id: key.key_id.clone(),
        head_sequence,
        entries,
    }))
}

fn default_protocol() -> String {
    "npm".to_owned()
}
