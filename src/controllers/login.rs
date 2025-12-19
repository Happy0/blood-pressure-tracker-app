use std::sync::Arc;

use axum::{response::Redirect, response:: Response, response::IntoResponse};
use openidconnect::{AuthenticationFlow, CsrfToken, EndpointMaybeSet, EndpointNotSet, EndpointSet, Nonce, PkceCodeChallenge, Scope, core::{CoreClient, CoreResponseType}};
use reqwest::StatusCode;
use tokio::join;
use tower_sessions::Session;

pub async fn login_handler(session: Session, client: Arc<CoreClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointMaybeSet, EndpointMaybeSet>>) -> Response {

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

    let csrf_insert = session.insert("OIDC_CSRF_TOKEN", csrf_state);
    let nonce_insert = session.insert("OIDC_NONCE", nonce);
    let pkce_verifier_insert = session.insert("OIDC_PKCE_VERIFIER", pkce_verifier);

    let result = join!(csrf_insert, nonce_insert, pkce_verifier_insert);

    match result {
        (Ok(_), Ok(_), Ok(_)) => {Redirect::temporary(authorize_url.as_str()).into_response()}
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error storing auth details into session").into_response()
    } 
    
}

pub async fn oidc_callback_handler(session: Session, client: Arc<CoreClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointMaybeSet, EndpointMaybeSet>>) -> Response {
    panic!("Panik!")
}