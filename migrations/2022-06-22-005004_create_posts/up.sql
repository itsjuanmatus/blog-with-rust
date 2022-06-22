-- Your SQL goes here
CREATE TABLE posts (
  id serial PRIMARY KEY,
  title varchar NOT NULL,
  slug varchar NOT NULL,
  body text NOT NULL
)