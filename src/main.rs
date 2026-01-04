use std::sync::Arc;
use std::env;

use axum::response::IntoResponse;
use axum::{Json, middleware};
use axum::{Router, routing::get, routing::post};
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, Session, SessionManagerLayer};

mod auth;
mod controllers;
mod repositories;

use crate::controllers::blood_pressure_reading::{add_reading, get_readings};
use crate::controllers::login::{
    auth_middleware, login_handler, logout_handler, oidc_callback_handler,
};
use crate::controllers::ocr::run_ocr;
use crate::repositories::session_repository::TowerSessionRepository;
use crate::repositories::sql_lite::sql_lite_blood_pressure_reading_repository::SqlLiteBloodPressureReadingRepository;
use sqlx::sqlite::SqlitePool;
use tower_sessions_sqlx_store::SqliteStore;

#[derive(Serialize)]
struct UserInfo {}

#[tokio::main]
async fn main() {
    let target_assets_directory = env::var("CLIENT_ASSETS_PATH").unwrap_or("client".to_string());

    let database_path = get_db_path();
    let sql_lite_pool = SqlitePool::connect(&database_path).await.unwrap();
    sqlx::migrate!("src/repositories/sql_lite/migrations")
        .run(&sql_lite_pool)
        .await
        .unwrap();

    let session_store = SqliteStore::new(sql_lite_pool.clone());
    session_store.migrate().await.unwrap();

    let shared_http_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "Could not construct http client")
        .unwrap();

    let oidc_client = auth::oidc::get_oidc_client(&shared_http_client)
        .await
        .unwrap();
    let shared_oidc_client = Arc::new(oidc_client);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!is_dev_mode())
        .with_http_only(true)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::weeks(2)));

    let serve_dir = ServeDir::new(&target_assets_directory).fallback(ServeFile::new(format!(
        "{}/index.html",
        target_assets_directory
    )));

    let blood_pressure_reading_repository = Arc::new(
        SqlLiteBloodPressureReadingRepository::from_pool(sql_lite_pool),
    );

    let app = Router::new()
        .route("/api/run-ocr", post(run_ocr))
        .route(
            "/login",
            get({
                let oidc_client = Arc::clone(&shared_oidc_client);

                move |session| login_handler(TowerSessionRepository::new(session), oidc_client)
            }),
        )
        .route("/logout", get(move |session| logout_handler(session)))
        .route(
            "/oidc-callback",
            get({
                let oidc_client = Arc::clone(&shared_oidc_client);
                move |session, params| {
                    oidc_callback_handler(
                        TowerSessionRepository::new(session),
                        oidc_client,
                        shared_http_client,
                        params,
                    )
                }
            }),
        )
        .route(
            "/api/user-info",
            get(async |_: Session| Json(UserInfo {}).into_response()),
        )
        .route(
            "/api/reading",
            post({
                let repository = Arc::clone(&blood_pressure_reading_repository);

                move |session, body| {
                    add_reading(repository, TowerSessionRepository::new(session), body)
                }
            }),
        )
        .route(
            "/api/reading",
            get({
                let repository = Arc::clone(&blood_pressure_reading_repository);

                move |session, params| {
                    get_readings(repository, TowerSessionRepository::new(session), params)
                }
            }),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .fallback_service(serve_dir)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server listen on port : {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn is_dev_mode() -> bool {
    let setting = env::var("DEV_MODE")
        .unwrap_or("false".to_string())
        .to_lowercase();

    match setting.as_str() {
        "true" => true,
        _ => false,
    }
}

fn get_db_path() -> String {
    env::var("BP_APP_DB_PATH").unwrap_or("sqlite:test.db".to_string())
}
