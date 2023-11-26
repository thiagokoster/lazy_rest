CREATE TABLE IF NOT EXISTS request (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        method TEXT NOT NULL,
        url TEXT NOT NULL
    )
 
