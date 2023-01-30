-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS scores (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        score REAL NOT NULL,
        custom TEXT,
        project_id INTEGER NOT NULL
    )