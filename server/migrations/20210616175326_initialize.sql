CREATE TABLE library (
    id SERIAL PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE device (
    id SERIAL PRIMARY KEY,
    charge integer NOT NULL,
    image bytea,
    library_id integer NOT NULL,
    CONSTRAINT fk_library FOREIGN KEY(library_id) REFERENCES library(id)
);
