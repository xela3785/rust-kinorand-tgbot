CREATE TABLE IF NOT EXISTS films (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    telegram_id INTEGER NOT NULL,
    username TEXT,
    title TEXT,
    original_title TEXT,
    year INTEGER,
    description TEXT,
    poster_url TEXT,
    kinopoisk_id INTEGER NOT NULL,
    kinopoisk_url TEXT NOT NULL,
    is_seen BOOLEAN DEFAULT FALSE,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE (kinopoisk_id, telegram_id)
);


CREATE INDEX IF NOT EXISTS idx_telegram_id on films(telegram_id);
CREATE INDEX IF NOT EXISTS idx_kinopoisk_id on films(kinopoisk_id);