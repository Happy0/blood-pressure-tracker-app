use axum::{Json, response::Response};
use chrono::{DateTime, Utc};
use tower_sessions::Session;

use crate::repositories::blood_pressure_readings_repository::BloodPressureReadingRepository;

pub struct BloodPressureReadingSubmission {
    pub systolic: i32,
    pub diastolic: i32,
    pub pulse: i32,
    pub taken: DateTime<Utc>
}

pub async fn add_reading<T: BloodPressureReadingRepository>(
    reading_repository: T,
    session: Session,
    Json(body): Json<BloodPressureReadingSubmission>) -> Response {
    panic!("Panik")
}