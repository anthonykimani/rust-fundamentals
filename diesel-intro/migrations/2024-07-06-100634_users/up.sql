-- Your SQL goes here
-- name, timestamp

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    firstname character varying(150) NOT NULL,
    age INTEGER NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL
)