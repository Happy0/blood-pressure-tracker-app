use std::sync::Arc;

use axum::{Json, response::{IntoResponse, Response}};
use reqwest::StatusCode;
use serde::Serialize;

use crate::repositories::{
    blood_pressure_readings_repository::{BloodPressureReadingRepository, RetrieveError},
    session_repository::{LoggedInSessionError, LoggedInSessionRepository, SessionRepository},
};

#[derive(Serialize)]
struct LatestWeightResponse {
    pub weight_kilograms: Option<f64>,
}

enum LatestWeightError {
    SessionError(LoggedInSessionError),
    DatebaseError(RetrieveError)
}

impl From<LoggedInSessionError> for LatestWeightError {
    fn from(value: LoggedInSessionError) -> Self {
        LatestWeightError::SessionError(value)
    }
}

impl From<RetrieveError> for LatestWeightError {
    fn from(value: RetrieveError) -> Self {
        LatestWeightError::DatebaseError(value)
    }
}

async fn get_latest_weight_from_database<T: BloodPressureReadingRepository, U: SessionRepository>(
    reading_repository: Arc<T>,
    session_repository: LoggedInSessionRepository<U>,
) -> Result<LatestWeightResponse, LatestWeightError> {
    let user_id = session_repository.get_oidc_user_subject().await?;
    let result = reading_repository.get_latest_weight(user_id).await?;

    Ok(LatestWeightResponse { weight_kilograms: result })
}

pub async fn get_latest_weight<T: BloodPressureReadingRepository, U: SessionRepository>(
    reading_repository: Arc<T>,
    session_repository: LoggedInSessionRepository<U>,
) -> Response {

    let result = get_latest_weight_from_database(
        reading_repository,
        session_repository,
    )
    .await;

    match result {
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
    }
}
