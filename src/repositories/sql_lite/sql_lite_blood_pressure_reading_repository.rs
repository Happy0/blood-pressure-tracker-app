use std::env;

use crate::repositories::blood_pressure_readings_repository::{
    BloodPressureReadingEntity, BloodPressureReadingRepository, RetrieveError, SaveError,
};
use chrono::DateTime;
use sqlx::{
    Error, Row,
    sqlite::{SqlitePool, SqliteRow},
};

pub struct SqlLiteBloodPressureReadingRepository {
    connection_pool: SqlitePool,
}

impl SqlLiteBloodPressureReadingRepository {
    pub async fn new(file_path: String) -> Result<SqlLiteBloodPressureReadingRepository, Error> {
        let pool = SqlitePool::connect(&file_path).await?;

        return Ok(SqlLiteBloodPressureReadingRepository {
            connection_pool: pool,
        });
    }

    fn get_db_path() -> String {
        env::var("BP_APP_DB_PATH").unwrap_or("sqlite:test.db".to_string())
    }

    pub async fn new_from_env() -> Result<SqlLiteBloodPressureReadingRepository, Error> {
        let path = Self::get_db_path();

        Self::new(path).await
    }
}

fn to_column_parse_error(column_name: &str) -> RetrieveError {
    RetrieveError::DeserializationError {
        description: format!("Could not deserialize {} column", column_name),
    }
}

fn deserialize_row(
    row: SqliteRow,
) -> Result<
    crate::repositories::blood_pressure_readings_repository::BloodPressureReadingEntity,
    RetrieveError,
> {
    let reading_id: String = row
        .try_get("reading_id")
        .map_err(|_| to_column_parse_error("reading_id"))?;
    let user_id: String = row
        .try_get("user_id")
        .map_err(|_| to_column_parse_error("user_id"))?;
    let systolic: i32 = row
        .try_get("systolic")
        .map_err(|_| to_column_parse_error("systolic"))?;
    let diastolic: i32 = row
        .try_get("diastolic")
        .map_err(|_| to_column_parse_error("diastolic"))?;
    let pulse: i32 = row
        .try_get("pulse")
        .map_err(|_| to_column_parse_error("pulse"))?;
    let taken_raw: String = row
        .try_get("taken")
        .map_err(|_| to_column_parse_error("taken"))?;
    let taken = DateTime::parse_from_rfc3339(&taken_raw)
        .map(|date| date.to_utc())
        .map_err(|_| to_column_parse_error("taken"))?;

    let result: BloodPressureReadingEntity = BloodPressureReadingEntity {
        reading_id,
        user_id,
        systolic,
        diastolic,
        pulse,
        taken: taken,
    };

    Ok(result)
}

impl BloodPressureReadingRepository for SqlLiteBloodPressureReadingRepository {
    async fn save(
        &self,
        entity: crate::repositories::blood_pressure_readings_repository::BloodPressureReadingEntity,
    ) -> Result<(), crate::repositories::blood_pressure_readings_repository::SaveError> {
        let result = sqlx::query(
            "INSERT into reading (reading_id, user_id, systolic, diastolic, pulse, taken) VALUES(?,?,?,?,?,?)"
        )
            .bind(entity.reading_id)
            .bind(entity.user_id)
            .bind(entity.systolic)
            .bind(entity.diastolic)
            .bind(entity.pulse)
            .bind(entity.taken.to_rfc3339())
            .execute(&self.connection_pool).await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(SaveError::LowLevelError {
                description: error.to_string(),
            }),
        }
    }

    async fn list(
        &self,
        user_id: String,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Result<
        Vec<crate::repositories::blood_pressure_readings_repository::BloodPressureReadingEntity>,
        crate::repositories::blood_pressure_readings_repository::RetrieveError,
    > {

        let query_result =
            sqlx::query("select * from reading WHERE user_id = ? AND taken >= ? AND taken <= ?")
                .bind(user_id)
                .bind(from.to_rfc3339())
                .bind(to.to_rfc3339())
                .fetch_all(&self.connection_pool)
                .await
                .map_err(|error| RetrieveError::LowLevelError {
                    description: error.to_string(),
                })?;

        let result: Result<
            Vec<BloodPressureReadingEntity>,
            crate::repositories::blood_pressure_readings_repository::RetrieveError,
        > = query_result
            .into_iter()
            .map(|row| deserialize_row(row))
            .collect();

        return result;
    }
}
