BEGIN;
CREATE TABLE airport_heliport (
    id          INTEGER PRIMARY KEY,
    designator  TEXT NOT NULL,
    name        TEXT NOT NULL,
    arp_x       REAL NOT NULL,
    arp_y       REAL NOT NULL,
    UNIQUE (designator)         
);
CREATE TABLE unit (
    id          INTEGER PRIMARY KEY,
    designator  TEXT NOT NULL,
    UNIQUE (designator)
);
COMMIT;