create table movies (
    id uuid not null,
    name text, 
    year int4, 
    was_good bool,
    PRIMARY KEY USING INDEX movies_id_pk
)