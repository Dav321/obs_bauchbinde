CREATE TABLE IF NOT EXISTS titles (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    label TEXT NOT NULL,
    UNIQUE (name, label)
);