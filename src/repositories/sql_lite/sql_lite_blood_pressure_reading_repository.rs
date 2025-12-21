use sqlx::{Error, sqlite::SqlitePool};
use crate::repositories::blood_pressure_readings_repository::{BloodPressureReadingRepository, SaveError};

pub struct SqlLiteBloodPressureReadingRepository {
    connection_pool: SqlitePool
}

impl SqlLiteBloodPressureReadingRepository {

    pub async fn new(file_path: String) -> Result<SqlLiteBloodPressureReadingRepository, Error> {
        let pool = SqlitePool::connect(&file_path).await?;

        return Ok(SqlLiteBloodPressureReadingRepository {
            connection_pool: pool
        })
    }
}

impl BloodPressureReadingRepository for SqlLiteBloodPressureReadingRepository {
    async fn save(&self, entity: crate::repositories::blood_pressure_readings_repository::BloodPressureReadingEntity) -> Result<(), crate::repositories::blood_pressure_readings_repository::SaveError> {

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
            Err(error) => Err(SaveError::LowLevelError { description: error.to_string() })
        }

    }

    async fn list(&self, user_id: String, from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>) -> Result<Vec<crate::repositories::blood_pressure_readings_repository::BloodPressureReadingEntity>, crate::repositories::blood_pressure_readings_repository::RetrieveError> {
        todo!()
    }
}