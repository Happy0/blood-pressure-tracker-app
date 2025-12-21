CREATE TABLE reading (
    reading_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    systolic INTEGER NOT NULL,
    diastolic INTEGER NOT NULL,
    pulse INTEGER NOT NULL,
    taken TEXT NOT NULL,
    PRIMARY KEY (reading_id, user_id)
);

CREATE INDEX idx_blood_pressure_reading_entity_user_id
ON reading (user_id);

CREATE INDEX idx_blood_pressure_reading_entity_taken
ON reading (taken);

CREATE INDEX idx_blood_pressure_reading_entity_user_taken
ON reading (user_id, taken);