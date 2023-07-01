CREATE TABLE IF NOT EXISTS redirect(
    url TEXT,
    accessed BIGINT,
    id TEXT UNIQUE PRIMARY
)