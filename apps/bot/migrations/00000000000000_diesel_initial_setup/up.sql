CREATE TABLE __diesel_schema_migrations (
    version VARCHAR(50) PRIMARY KEY NOT NULL,
    run_on TIMESTAMP NOT NULL DEFAULT NOW()
);
