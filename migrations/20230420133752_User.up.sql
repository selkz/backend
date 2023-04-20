-- Add up migration script here
CREATE TABLE IF NOT EXISTS Users (
    Id varchar(255) NOT NULL UNIQUE,
    Username varchar(255) NOT NULL,
    Email varchar(255) NOT NULL UNIQUE,
    Password varchar(255) NOT NULL,
    SessionToken varchar(255) NOT NULL UNIQUE
);