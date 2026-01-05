use std::sync::Arc;

use axum::{
    Json,
    extract::Query,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repositories::{
    blood_pressure_readings_repository::{
        BloodPressureReadingEntity, BloodPressureReadingRepository,
    },
    session_repository::{LoggedInSessionRepository, SessionRepository},
};

#[derive(Deserialize)]
pub struct BloodPressureReadingSubmission {
    pub systolic: i32,
    pub diastolic: i32,
    pub pulse: i32,
    pub weight_kilograms: Option<f64>,
    pub taken: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct BloodPressureReadingResponse {
    pub systolic: i32,
    pub diastolic: i32,
    pub pulse: i32,
    pub weight_kilograms: Option<f64>,
    pub taken: DateTime<Utc>,
    pub id: String,
}


#[derive(Deserialize)]
pub struct GetReadingQueryParameters {
    pub from_inclusive: DateTime<Utc>,
    pub to_inclusive: DateTime<Utc>,
}

async fn add_reading_to_database<T: BloodPressureReadingRepository, U: SessionRepository>(
    reading_repository: &Arc<T>,
    session_repository: LoggedInSessionRepository<U>,
    reading: BloodPressureReadingSubmission,
) -> Result<(), String> {
    let user_id = session_repository
        .get_oidc_user_subject()
        .await
        .map_err(|_| "Could not access user ID from")?;

    let blood_pressure_reading_id = Uuid::now_v7().to_string();

    let entity: BloodPressureReadingEntity = BloodPressureReadingEntity {
        reading_id: blood_pressure_reading_id,
        user_id: user_id,
        systolic: reading.systolic,
        diastolic: reading.diastolic,
        pulse: reading.pulse,
        weight_kilograms: reading.weight_kilograms,
        taken: reading.taken,
    };

    reading_repository
        .save(entity)
        .await
        .map_err(|_| "Could not save to database")?;

    Ok(())
}

pub async fn add_reading<T: BloodPressureReadingRepository, U: SessionRepository>(
    reading_repository: Arc<T>,
    session_repository: LoggedInSessionRepository<U>,
    Json(body): Json<BloodPressureReadingSubmission>,
) -> Response {
    let result = add_reading_to_database(&reading_repository, session_repository, body).await;

    match result {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

fn to_api_representation(entity: BloodPressureReadingEntity) -> BloodPressureReadingResponse {
    BloodPressureReadingResponse {
        systolic: entity.systolic,
        diastolic: entity.diastolic,
        pulse: entity.pulse,
        taken: entity.taken,
        weight_kilograms: entity.weight_kilograms,
        id: entity.reading_id,
    }
}

async fn get_readings_from_database<T: BloodPressureReadingRepository, U: SessionRepository>(
    reading_repository: Arc<T>,
    session_repository: LoggedInSessionRepository<U>,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<BloodPressureReadingResponse>, String> {
    let user_id = session_repository
        .get_oidc_user_subject()
        .await
        .map_err(|_| "Could not access user ID from session")?;

    let db_result = reading_repository
        .list(user_id, from, to)
        .await
        .map_err(|_| "Error retrieving from database")?;

    let result: Vec<BloodPressureReadingResponse> = db_result
        .into_iter()
        .map(|entity| to_api_representation(entity))
        .collect();

    Ok(result)
}

pub async fn get_readings<T: BloodPressureReadingRepository, U: SessionRepository>(
    reading_repository: Arc<T>,
    session_repository: LoggedInSessionRepository<U>,
    query: Query<GetReadingQueryParameters>,
) -> Response {
    // TODO: validate query parameters

    let result = get_readings_from_database(
        reading_repository,
        session_repository,
        query.from_inclusive,
        query.to_inclusive,
    )
    .await;

    match result {
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
    }
}