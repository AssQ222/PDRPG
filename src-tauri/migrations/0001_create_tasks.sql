-- Migration 0001: Create tasks table
-- This migration creates the initial tasks table with all required fields

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Index na kolumnie completed dla szybkich zapytań o zadania ukończone/nieukończone
CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed);

-- Index na kolumnie created_at dla sortowania zadań po dacie utworzenia
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);

-- Index na kolumnie updated_at dla sortowania zadań po dacie modyfikacji
CREATE INDEX IF NOT EXISTS idx_tasks_updated_at ON tasks(updated_at); 