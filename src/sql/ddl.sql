BEGIN;
CREATE TABLE airport_heliport (
    id          TEXT PRIMARY KEY,
    designator  TEXT NOT NULL,
    name        TEXT NOT NULL,
    arp_x       REAL NOT NULL,
    arp_y       REAL NOT NULL,
    UNIQUE (designator)         
);

CREATE TABLE runway (
    id                  TEXT PRIMARY KEY,
    designator          TEXT NOT NULL,
    associated_airport   TEXT NOT NULL,
    UNIQUE (associated_airport, designator),
    FOREIGN KEY (associated_airport) REFERENCES airport_heliport(id)
);

CREATE TABLE runway_end (
    id              INTEGER PRIMARY KEY,          
    runway          TEXT NOT NULL,
    designator      TEXT NOT NULL,
    base_end        BOOLEAN NOT NULL,
    UNIQUE (runway, designator),
    FOREIGN KEY (runway) REFERENCES runway(id)
);

CREATE TABLE unit (
    id          TEXT PRIMARY KEY,
    designator  TEXT NOT NULL,
    UNIQUE (designator)
);
COMMIT;