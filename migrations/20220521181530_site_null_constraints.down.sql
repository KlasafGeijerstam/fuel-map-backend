-- Add down migration script here
ALTER TABLE site
    ALTER COLUMN address
        DROP DEFAULT;
ALTER TABLE site
    ALTER COLUMN address
        DROP NOT NULL;

ALTER TABLE site
    ALTER COLUMN lat
        DROP DEFAULT;
ALTER TABLE site
    ALTER COLUMN lat
        DROP NOT NULL;

ALTER TABLE site
    ALTER COLUMN lng
        DROP DEFAULT;
ALTER TABLE site
    ALTER COLUMN lng
        DROP NOT NULL;
