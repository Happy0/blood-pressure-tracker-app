use chrono::{DateTime, Utc};

pub struct BloodPressureReadingEntity {
    pub reading_id: String,
    pub user_id: String,
    pub systolic: i32,
    pub diastolic: i32,
    pub pulse: i32,
    pub taken: DateTime<Utc>,
}

#[derive(Debug)]
pub enum SaveError {
    LowLevelError { description: String },
}

#[derive(Debug)]
pub enum RetrieveError {
    LowLevelError { description: String },
    DeserializationError { description: String },
}

pub trait BloodPressureReadingRepository {
    async fn save(&self, entity: BloodPressureReadingEntity) -> Result<(), SaveError>;

    /**
     * Retrieves the list of readings from the user in descending order of the date and time they were taken
     */
    async fn list(
        &self,
        user_id: String,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<BloodPressureReadingEntity>, RetrieveError>;
}
