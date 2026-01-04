use std::sync::Arc;

use crate::repositories::session_repository::SessionRepository;
use axum::{
    extract::{Query, Request},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use openidconnect::{
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, CsrfToken, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, Nonce, OAuth2TokenResponse, PkceCodeChallenge, Scope,
    TokenResponse,
    core::{CoreClient, CoreResponseType},
    reqwest::Client,
};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::join;
use tower_sessions::Session;

const OIDC_CSRF_TOKEN_KEY: &str = "OIDC_CSRF_TOKEN";
const OIDC_NONCE_KEY: &str = "OIDC_NONCE";
const OIDC_PKCE_VERIFIER_KEY: &str = "OIDC_PKCE_VERIFIER";

const SUBJECT_SESSION_KEY: &str = "OIDC_SUBJECT_KEY";

struct OpenIdDetails {
    subject: String,
}

#[derive(Deserialize)]
pub struct CallBackParameters {
    code: String,
    state: String,
}

pub async fn login_handler<T: SessionRepository>(
    session_repository: T,
    client: Arc<
        CoreClient<
            EndpointSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointMaybeSet,
            EndpointMaybeSet,
        >,
    >,
) -> Response {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state, nonce) = client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // This example is requesting access to the the user's profile including email.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let csrf_insert = session_repository.save_oidc_crsf_token(csrf_state.into_secret());

    let nonce_insert = session_repository.save_oidc_nonce_key(nonce);
    let pkce_verifier_insert = session_repository.save_pkce_verifier(pkce_verifier);

    let result = join!(csrf_insert, nonce_insert, pkce_verifier_insert);

    match result {
        (Ok(_), Ok(_), Ok(_)) => Redirect::temporary(authorize_url.as_str()).into_response(),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error storing auth details into session",
        )
            .into_response(),
    }
}

async fn get_user_details<T: SessionRepository>(
    session_repository: &T,
    client: Arc<
        CoreClient<
            EndpointSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointMaybeSet,
            EndpointMaybeSet,
        >,
    >,
    http_client: Client,
    authorization_code: AuthorizationCode,
    state: String,
) -> Result<OpenIdDetails, String> {
    let csrf_token = session_repository.get_oidc_crsf_token();
    let nonce = session_repository.get_oidc_nonce_key();
    let pkce_verifier = session_repository.get_pkce_verifier();

    let results = join!(csrf_token, nonce, pkce_verifier);

    match results {
        (Ok(Some(csrf_token)), Ok(Some(nonce)), Ok(Some(verifier))) => {
            let exchange_code = client
                .exchange_code(authorization_code)
                .map_err(|_| "Could not exchange code")?;
            let token_response = exchange_code
                .set_pkce_verifier(verifier)
                .request_async(&http_client)
                .await
                .map_err(|err| {
                    println!("err {:?}", err);
                    "Could not make call with exchange code"
                })?;
            let id_token_verifier = client.id_token_verifier();
            let token = token_response
                .id_token()
                .ok_or_else(|| "Service did not return an ID token")?;

            let claims = token
                .claims(&id_token_verifier, &nonce)
                .map_err(|_| "Could not verify claims")?;

            let statearoonie = CsrfToken::new(state);

            if csrf_token != *statearoonie.secret() {
                return Err("Could not verify CSRF token.".to_string());
            }

            if let Some(expected_token_hash) = claims.access_token_hash() {
                let token_signing_algorithm = token
                    .signing_alg()
                    .map_err(|_| "Problem with token signing algorithm")?;
                let token_signing_key = token
                    .signing_key(&id_token_verifier)
                    .map_err(|_| "Problem with signing key")?;

                let access_token = token_response.access_token();

                let actual_token_hash = AccessTokenHash::from_token(
                    &access_token,
                    token_signing_algorithm,
                    token_signing_key,
                )
                .map_err(|_| "Problem creating token hash")?;

                if actual_token_hash != *expected_token_hash {
                    return Err("Invalid access token".to_string());
                }
            }

            let subject: &openidconnect::SubjectIdentifier = claims.subject();

            return Ok(OpenIdDetails {
                subject: subject.as_str().to_string(),
            });
        }
        _ => Err("Error getting auth details from session".to_string()),
    }
}

pub async fn oidc_callback_handler<T: SessionRepository>(
    session: T,
    oidc_client: Arc<
        CoreClient<
            EndpointSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointMaybeSet,
            EndpointMaybeSet,
        >,
    >,
    http_client: Client,
    query_params: Query<CallBackParameters>,
) -> Response {
    let CallBackParameters { code, state } = query_params.0;

    let authorization_code = AuthorizationCode::new(code);

    let details = get_user_details(
        &session,
        oidc_client,
        http_client,
        authorization_code,
        state,
    )
    .await;

    match details {
        Ok(user_details) => {
            let store_subject_result = session.save_oidc_user_subject(user_details.subject).await;

            let clear_result = session.clear_oidc_flow_details().await;

            if store_subject_result.is_err() || clear_result.is_err() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not start session.",
                )
                    .into_response();
            }

            Redirect::temporary("/").into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication not successful.",
        )
            .into_response(),
    }
}

pub async fn logout_handler(session: Session) -> Response {
    let result = session.flush().await;

    match result {
        Ok(_) => Redirect::temporary("/").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn auth_middleware(session: Session, request: Request, next: Next) -> Response {
    let is_api_request = request.uri().to_string().starts_with("/api/");

    if !is_api_request {
        return next.run(request).await;
    }

    let result = session.get::<String>(SUBJECT_SESSION_KEY).await;

    match result {
        Ok(Some(_)) => return next.run(request).await,
        _ => return (StatusCode::UNAUTHORIZED).into_response(),
    }
}
