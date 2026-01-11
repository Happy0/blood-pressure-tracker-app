use std::sync::Arc;

use axum::{extract::Query, response::Response};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tower_sessions::Session;

use crate::repositories::blood_pressure_readings_repository::BloodPressureReadingRepository;

#[derive(Deserialize)]
pub struct GetCsvExportQueryParameters {
    pub from_inclusive: DateTime<Utc>,
    pub to_inclusive: DateTime<Utc>,
}

fn get_reading_csv_export<T: BloodPressureReadingRepository>(
    _reading_repository: Arc<T>,
    _session_repository: Arc<Session>,
    _query_params: Query<GetCsvExportQueryParameters>,
) -> Response {
    panic!("Panik")
}
