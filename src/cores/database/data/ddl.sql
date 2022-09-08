CREATE TABLE states (
    id SERIAL NOT NULL PRIMARY KEY,
    code VARCHAR(20) NOT NULL,
    description TEXT,
    webhooks TEXT[],
    create_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    update_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc')
);