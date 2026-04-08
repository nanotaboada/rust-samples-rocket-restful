CREATE TABLE IF NOT EXISTS players (
    id            TEXT    NOT NULL PRIMARY KEY,
    first_name    TEXT    NOT NULL,
    middle_name   TEXT    NOT NULL,
    last_name     TEXT    NOT NULL,
    date_of_birth TEXT    NOT NULL,
    squad_number  INTEGER NOT NULL UNIQUE,
    position      TEXT    NOT NULL,
    abbr_position TEXT    NOT NULL,
    team          TEXT    NOT NULL,
    league        TEXT    NOT NULL,
    starting11    INTEGER NOT NULL CHECK (starting11 IN (0, 1))
);
