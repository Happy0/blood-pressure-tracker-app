use std::sync::Arc;

use axum::{
    extract::{Query, Request}, middleware::Next, response::{IntoResponse, Redirect, Response}
};
use openidconnect::{
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, CsrfToken,
    EndpointMaybeSet, EndpointNotSet, EndpointSet, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, Scope, TokenResponse,
    core::{CoreClient, CoreResponseType},
};
use reqwest::{ StatusCode};
use tokio::join;
use tower_sessions::Session;

const OIDC_CSRF_TOKEN_KEY: &str = "OIDC_CSRF_TOKEN";
const OIDC_NONCE_KEY: &str = "OIDC_CSRF_TOKEN";
const OIDC_PKCE_VERIFIER_KEY: &str = "OIDC_PKCE_VERIFIER";

const SUBJECT_SESSION_KEY: &str = "OIDC_SUBJECT_KEY";

struct OpenIdDetails {
    subject: String,
}

pub async fn login_handler(
    session: Session,
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
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let csrf_insert = session.insert(OIDC_CSRF_TOKEN_KEY, csrf_state);
    let nonce_insert = session.insert(OIDC_NONCE_KEY, nonce);
    let pkce_verifier_insert = session.insert(OIDC_PKCE_VERIFIER_KEY, pkce_verifier);

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

async fn get_user_details(
    session: &Session,
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
    authorization_code: AuthorizationCode,
    state: String,
) -> Result<OpenIdDetails, String> {
    // TODO: share client between handler calls
    let http_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "Could not construct http client")?;

    let csrf_token = session.get::<String>(OIDC_CSRF_TOKEN_KEY);
    let nonce = session.get::<Nonce>(OIDC_NONCE_KEY);
    let pkce_verifier = session.get::<PkceCodeVerifier>(OIDC_PKCE_VERIFIER_KEY);

    // TODO: delete verification info above from session

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
                .map_err(|_| "Could not make call with exchange code")?;
            let id_token_verifier = client.id_token_verifier();
            let token = token_response
                .id_token()
                .ok_or_else(|| "Service did not return an ID token")?;
            let claims = token
                .claims(&id_token_verifier, &nonce)
                .map_err(|_| "Could not verify claims")?;

            if (csrf_token != state) {
                return Err("Could not verify CSRF token.".to_string());
            }

            let expected_token_hash = claims
                .access_token_hash()
                .ok_or_else(|| "No token hash".to_string())?;

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

            let subject: &openidconnect::SubjectIdentifier = claims.subject();

            return Ok(OpenIdDetails {
                subject: subject.as_str().to_string(),
            });
        }
        _ => Err("Error getting auth details from session".to_string()),
    }
}

pub async fn oidc_callback_handler(
    session: Session,
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
    code: Query<String>,
    state: Query<String>,
) -> Response {
    let authorization_code = AuthorizationCode::new(code.0);

    let details = get_user_details(&session, client, authorization_code, state.0).await;

    match details {
        Ok(user_details) => {
            let store_subject_result = session
                .insert(SUBJECT_SESSION_KEY, user_details.subject)
                .await;

            let d1 = session.remove::<String>(OIDC_CSRF_TOKEN_KEY);
            let d2 = session.remove::<Nonce>(OIDC_NONCE_KEY);
            let d3 = session.remove::<PkceCodeVerifier>(OIDC_PKCE_VERIFIER_KEY);

            _ = join!(d1,d2,d3);

            if store_subject_result.is_err() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not start session.",
                )
                    .into_response();
            }

            println!("Logged in!");
            Redirect::temporary("/").into_response()
        }
        Err(err_details) => {
            println!("Err: {:?}", err_details);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication not successful.",
            )
                .into_response()
        }
    }
}

pub async fn auth_middleware(session: Session, request: Request, next: Next) -> Response {
    let result = session.get::<String>(SUBJECT_SESSION_KEY).await;

    match result {
        Ok(Some(_)) => return next.run(request).await,
        _ => return (StatusCode::UNAUTHORIZED).into_response()
    }
}
