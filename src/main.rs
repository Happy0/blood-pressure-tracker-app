
use std::env;

use axum::{
    Router, routing::post
};
use tower_http::services::{ServeDir, ServeFile};

mod controllers;
use crate::controllers::ocr::run_ocr;

#[tokio::main]
async fn main() {
    let target_assets_directory = env::var("CLIENT_ASSETS_PATH").unwrap_or("client".to_string());
    let serve_dir = ServeDir::new(&target_assets_directory)
        .fallback(ServeFile::new(format!("{}/index.html", target_assets_directory)));

    let app = Router::new()
        .route("/run-ocr", post(run_ocr))
        .fallback_service(serve_dir);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server listen on port : {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
