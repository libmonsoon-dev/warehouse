-- Your SQL goes here
CREATE TABLE "users"
(
    "id"            UUID         NOT NULL PRIMARY KEY,
    "first_name"    VARCHAR(256) NOT NULL,
    "last_name"     VARCHAR(256) NOT NULL,
    "email"         VARCHAR(256) NOT NULL UNIQUE,
    "password_hash" VARCHAR(256) NOT NULL
);

