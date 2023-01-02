-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS scores (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        score INTEGER NOT NULL,
        project_id INTEGER NOT NULL
    )