#!/usr/bin/env bash

mkdir -p /data
sqlite3 /data/bp_database.db "VACUUM;"

export BP_APP_DB_PATH="sqlite:///data/bp_database.db"

blood-pressure-tracker-app