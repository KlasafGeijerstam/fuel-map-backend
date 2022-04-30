-- Add up migration script here
CREATE TABLE site (
    id SERIAL PRIMARY KEY,
    address TEXT,
    lat TEXT,
    lng TEXT
);
