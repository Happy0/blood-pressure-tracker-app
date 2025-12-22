#!/usr/bin/env bash

DATABASE_URL="sqlite:///data/bp_database.db" sqlx database create
DATABASE_URL="sqlite:///data/bp_database.db" sqlx migrate run

export BP_APP_DB_PATH="sqlite:///data/bp_database.db"

blood-pressure-tracker-app