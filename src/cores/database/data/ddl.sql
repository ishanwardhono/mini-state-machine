CREATE TABLE states (
    id SERIAL NOT NULL PRIMARY KEY,
    code VARCHAR(20) NOT NULL UNIQUE,
    description TEXT,
    webhooks TEXT[],
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc')
);

CREATE TYPE role AS ENUM ('ADMIN', 'BUSINESS_CLIENT');

CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE,
    "role" role NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc')
);