CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE role AS ENUM ('ADMIN', 'BUSINESS_CLIENT');

CREATE TABLE users (
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    username VARCHAR(25) NOT NULL UNIQUE,
    "role" role NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    create_by uuid NOT NULL,
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_by uuid NOT NULL
);

CREATE TABLE states (
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    code VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    webhooks TEXT[],
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    create_by uuid NOT NULL,
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_by uuid NOT NULL
);

CREATE TABLE business (
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    code VARCHAR(25) NOT NULL UNIQUE,
    description TEXT,
    is_active boolean,
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    create_by uuid NOT NULL,
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_by uuid NOT NULL
);

CREATE TABLE flows (
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    business VARCHAR(25) NOT NULL,
    state VARCHAR(50) NOT NULL,
    is_initial_state boolean,
    next_states TEXT[],
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    create_by uuid NOT NULL,
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_by uuid NOT NULL
)