use std::env;
use std::sync::Arc;

use axum::{Json, middleware};
use axum::response::IntoResponse;
use axum::{Router, routing::get, routing::post};
use reqwest::StatusCode;
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

mod auth;
mod controllers;
use crate::controllers::login::{auth_middleware, login_handler, oidc_callback_handler};
use crate::controllers::ocr::run_ocr;

#[derive(Serialize)]
struct UserInfo {}

#[tokio::main]
async fn main() {
    let target_assets_directory = env::var("CLIENT_ASSETS_PATH").unwrap_or("client".to_string());
    let oidc_client = auth::oidc::get_oidc_client().await.unwrap();
    let shared_oidc_client = Arc::new(oidc_client);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        // TODO: configure via environment
        .with_secure(false)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(Duration::weeks(2)));

    let serve_dir = ServeDir::new(&target_assets_directory).fallback(ServeFile::new(format!(
        "{}/index.html",
        target_assets_directory
    )));

    let app = Router::new()
        .route("/api/run-ocr", post(run_ocr))
        .route(
            "/api/login",
            get({
                let oidc_client = Arc::clone(&shared_oidc_client);
                move |session| login_handler(session, oidc_client)
            }),
        )
        .route(
            "/oidc-callback",
            get({
                let oidc_client = Arc::clone(&shared_oidc_client);
                move |session, code, state| oidc_callback_handler(session, oidc_client, code, state)
            }),
        )
        .route(
            "/api/user-info",
            get({
                async |session: Session| {
                    Json(UserInfo{}).into_response()
                }
            })
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .fallback_service(serve_dir)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server listen on port : {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
