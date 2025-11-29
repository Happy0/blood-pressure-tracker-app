use std::sync::Arc;

use axum::{
    Json, Router, body::Bytes, extract::Multipart, http::StatusCode, routing::post
};
use axum_macros::debug_handler;
use bpm_ocr::{BloodPressureReadingExtractor, debug::UnsafeTempFolderDebugger};
use serde::Serialize;
use tokio::task;

#[derive(Serialize)]
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
    let field = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)?;

    match field  {
        Some(field) => {
            field.name()
                .filter(|n| *n == "image")
                .ok_or(StatusCode::BAD_REQUEST)?;
            
            let file_contents: Bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            let file_contents_vec = file_contents.to_vec();

            task::spawn_blocking( move || {
                let debugger: UnsafeTempFolderDebugger = UnsafeTempFolderDebugger::using_timestamp_folder_name(true);
                let blood_pressure_extractor = BloodPressureReadingExtractor::new(debugger);

                blood_pressure_extractor.get_reading_from_buffer(file_contents_vec)
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
    let app = Router::new().route("/run-ocr", post(run_ocr));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server listen on port : {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}
