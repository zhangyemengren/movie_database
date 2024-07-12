-- Add migration script here
CREATE TABLE `weekly_trending_movies`
(
    backdrop_path VARCHAR(255),
    id INT UNSIGNED PRIMARY KEY,
    title VARCHAR(255),
    original_title VARCHAR(255),
    overview TEXT,
    poster_path VARCHAR(255),
    adult BOOLEAN,
    original_language VARCHAR(10),
    genre_ids JSON,
    popularity FLOAT,
    release_date DATE,
    vote_average FLOAT,
    vote_count INT UNSIGNED
)