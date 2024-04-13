-- Add migration script here
CREATE TABLE repositories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path varchar(255) NOT NULL UNIQUE
);
