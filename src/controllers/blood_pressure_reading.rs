use std::sync::Arc;

use axum::{Json, response::{IntoResponse, Response}};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;

use crate::repositories::blood_pressure_readings_repository::{BloodPressureReadingEntity, BloodPressureReadingRepository};

const SUBJECT_SESSION_KEY: &str = "OIDC_SUBJECT_KEY";

#[derive(Deserialize)]
pub struct BloodPressureReadingSubmission {
    pub systolic: i32,
    pub diastolic: i32,
    pub pulse: i32,
    pub taken: DateTime<Utc>
}

async fn add_reading_to_database<T: BloodPressureReadingRepository>(
    reading_repository: &Arc<T>,
    session: Session, reading: BloodPressureReadingSubmission) -> Result<(), String> {
    let subject = session.get::<String>(SUBJECT_SESSION_KEY).await.map_err(|_| "Could not access session")?;

    let user_id = subject.ok_or("Missing user ID in session")?;

    let blood_pressure_reading_id = Uuid::now_v7().to_string();

    let entity: BloodPressureReadingEntity = BloodPressureReadingEntity { 
        reading_id:blood_pressure_reading_id,
        user_id: user_id,
        systolic: reading.systolic,
        diastolic: reading.diastolic,
        pulse: reading.pulse, taken: reading.taken 
    };

    reading_repository.save(entity).await.map_err(|_| "Could not save to database")?;

    Ok(())
}

pub async fn add_reading<T: BloodPressureReadingRepository>(
    reading_repository: Arc<T>,
    session: Session,
    Json(body): Json<BloodPressureReadingSubmission>) -> Response {

    let result = add_reading_to_database(&reading_repository, session, body).await;

    match result {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }

}