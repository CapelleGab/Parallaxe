ALTER TABLE roots ADD COLUMN is_primary INTEGER NOT NULL DEFAULT 0 CHECK (is_primary IN (0, 1));

-- the library folder: at most one per library
CREATE UNIQUE INDEX roots_one_primary ON roots (library_id) WHERE is_primary = 1;
