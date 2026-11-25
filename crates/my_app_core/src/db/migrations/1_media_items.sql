CREATE TABLE media_items (
    id SERIAL PRIMARY KEY,          -- IMDb ID (e.g., tt1234567)
    imdb_id TEXT,
    media_type TEXT NOT NULL,     -- 'movie' or 'series'
    title TEXT NOT NULL,
    overview TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    release_year INT,
    rating REAL
);

INSERT INTO media_items (media_type, title) VALUES (
    "movie", "test movie"
)