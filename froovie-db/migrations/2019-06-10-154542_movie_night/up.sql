CREATE TABLE movie_night
(
    id         SERIAL PRIMARY KEY,
    movie_time DATE
);

CREATE TABLE movies
(
    id         SERIAL PRIMARY KEY,
    moviedb_id INTEGER NOT NULL ,
    title      TEXT NOT NULL ,
    description TEXT,
    image_url   TEXT
);


CREATE TABLE user_selections
(
    id         SERIAL PRIMARY KEY,
    user_id  INTEGER REFERENCES users (id)       NOT NULL,
    movie_id INTEGER REFERENCES movies (id)      NOT NULL,
    chosen BOOLEAN DEFAULT FALSE NOT NULL
)
