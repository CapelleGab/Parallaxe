-- each scan of a root bumps its generation; files seen during that scan get the same number
ALTER TABLE roots ADD COLUMN scan_generation INTEGER NOT NULL DEFAULT 0;
ALTER TABLE assets ADD COLUMN seen_generation INTEGER NOT NULL DEFAULT 0;

CREATE INDEX assets_root_generation ON assets (root_id, seen_generation);
