-- Add up migration script here
ALTER TABLE site
    ALTER COLUMN address
        SET DEFAULT '';
ALTER TABLE site
    ALTER COLUMN address
        SET NOT NULL;

ALTER TABLE site
    ALTER COLUMN lat
        SET DEFAULT '';
ALTER TABLE site
    ALTER COLUMN lat
        SET NOT NULL;

ALTER TABLE site
    ALTER COLUMN lng
        SET DEFAULT '';
ALTER TABLE site
    ALTER COLUMN lng
        SET NOT NULL;
