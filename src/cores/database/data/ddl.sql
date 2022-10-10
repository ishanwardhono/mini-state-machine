CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE states (
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    code VARCHAR(20) NOT NULL UNIQUE,
    description TEXT,
    webhooks TEXT[],
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    create_by uuid NOT NULL,
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_by uuid NOT NULL
);

CREATE TYPE role AS ENUM ('ADMIN', 'BUSINESS_CLIENT');

CREATE TABLE users (
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE,
    "role" role NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    create_by uuid NOT NULL,
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_by uuid NOT NULL
);