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

CREATE TABLE unit (
    id          TEXT PRIMARY KEY,
    designator  TEXT NOT NULL,
    UNIQUE (designator)
);
COMMIT;