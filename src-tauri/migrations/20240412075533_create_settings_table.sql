-- Add migration script here
CREATE TABLE settings (
    startup BOOL NOT NULL DEFAULT true,
    notifications BOOL NOT NULL DEFAULT true,
    all_authors BOOL NOT NULL DEFAULT false
);
