CREATE TABLE asthobin (
    id TEXT NOT NULL,
    content TEXT NOT NULL,
    language TEXT NOT NULL,
    time BIGINT NOT NULL CHECK (time >= 0),
    CONSTRAINT asthobin_pk PRIMARY KEY (id)
);