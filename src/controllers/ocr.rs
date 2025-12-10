use std::fs::read;

use axum::{Json, body::Bytes, extract::Multipart, http::StatusCode};
use bpm_ocr::{get_reading_from_buffer, models::{BloodPressureReading, DebuggerTrace, ProcessingError}};
use serde::Serialize;
use tokio::task;


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
    },
    UnlikelyReading {
        systolic: i32,
        diastolic: i32,
        pulse: i32
    }
}

// Sometimes one of the digits isn't detected on one of the rows until the device is tilted slightly / removed from glare, etc
fn is_unlikely_reading(reading: &BloodPressureReading) -> bool {
    // They ded (or ded-ish)
    if reading.systolic < 50 { return true; }
    else if reading.diastolic < 40 {return true; }

    // Or mebbe they just don't have a pulse - like Dick Cheney
    else if reading.pulse < 40 { return true; }
    else { return false; }
}

fn map_ocr_result(reading: Result<BloodPressureReading, ProcessingError>) -> BloodPressureReadingResponse {
    match reading {
        Err(_) => BloodPressureReadingResponse::ReadingError {description: "Could not detect reading.".to_string()},
        Ok(reading) if is_unlikely_reading(&reading) => BloodPressureReadingResponse::UnlikelyReading { systolic: reading.systolic, diastolic: reading.diastolic, pulse: reading.pulse },
        Ok(reading) => BloodPressureReadingResponse::Reading { systolic: reading.systolic, diastolic: reading.diastolic, pulse: reading.pulse }
    }

}

pub async fn run_ocr(mut multipart: Multipart) -> Result<Json<BloodPressureReadingResponse>, StatusCode> {
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
                let debugger_trace = DebuggerTrace::no_debug_session();
                let ocr_result = get_reading_from_buffer(file_contents_vec, debugger_trace);

                println!("{:?}", ocr_result);
                
                let result = map_ocr_result(ocr_result );

                Json(result)
                }).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        },
        None => {
            Err(StatusCode::BAD_REQUEST)
        }
    }


}