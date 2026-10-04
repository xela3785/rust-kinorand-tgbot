CREATE TABLE IF NOT EXISTS chat_members (
    chat_id INTEGER NOT NULL,
    telegram_id INTEGER NOT NULL,
    username TEXT,
    joined_at TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (chat_id, telegram_id)
);

CREATE INDEX IF NOT EXISTS idx_is_seen ON films (is_seen);
CREATE INDEX IF NOT EXISTS idx_chat_members_chat ON chat_members(chat_id);
CREATE INDEX IF NOT EXISTS idx_chat_members_user ON chat_members(telegram_id);
