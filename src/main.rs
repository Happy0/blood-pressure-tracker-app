use std::sync::Arc;

use axum::{
    Json, Router, body::Bytes, extract::{DefaultBodyLimit, Multipart}, http::StatusCode, routing::post
};
use bpm_ocr::{get_reading_from_buffer, models::DebuggerTrace};
use serde::Serialize;
use tokio::task;
use tower_http::services::ServeDir;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum BloodPressureReadingResponse {
    Reading {
        systolic: i32,
        diastolic: i32,
        pulse: i32
    }, 
    ReadingError {
        description: String
    }
}

async fn run_ocr(mut multipart: Multipart) -> Result<Json<BloodPressureReadingResponse>, StatusCode> {
    let field = multipart.next_field().await.map_err(|ee|
         {
            println!("{:?}", ee);
            StatusCode::BAD_REQUEST
        })?;

    match field  {
        Some(field) => {
            field.name()
                .filter(|n| *n == "image")
                .ok_or(StatusCode::BAD_REQUEST)?;
            
            let file_contents: Bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            let file_contents_vec = file_contents.to_vec();

            task::spawn_blocking( move || {
                let debugger_trace = DebuggerTrace::temp_folder_session_uuid();
                let ocr_result = get_reading_from_buffer(file_contents_vec, debugger_trace);

                println!("{:?}", ocr_result);
                
                ocr_result 
                    .map(|reading| Json(BloodPressureReadingResponse::Reading { systolic: reading.systolic, diastolic: reading.diastolic, pulse: reading.pulse })
                    )
                    .or_else(|err| Ok(Json(BloodPressureReadingResponse::ReadingError { description: "erroraroonie".to_string() })))
                }).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).flatten()
        },
        None => {
            Err(StatusCode::BAD_REQUEST)
        }
    }


}

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("client");

    let app = Router::new()
        .route("/run-ocr", post(run_ocr))
        .fallback_service(serve_dir);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server listen on port : {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();

}
