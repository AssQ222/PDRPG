-- Migration 0002: Create habits and habit_entries tables (SIMPLIFIED)
-- This migration creates basic tables for the habit tracking module

-- Create habits table (simplified)
CREATE TABLE IF NOT EXISTS habits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    habit_type TEXT NOT NULL,
    target_value INTEGER,
    current_streak INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Create habit_entries table (simplified) 
CREATE TABLE IF NOT EXISTS habit_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    habit_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    completed INTEGER DEFAULT 0,
    value INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL
); 