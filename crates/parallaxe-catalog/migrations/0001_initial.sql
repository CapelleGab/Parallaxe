CREATE TABLE libraries (
    id         TEXT    NOT NULL PRIMARY KEY,
    name       TEXT    NOT NULL CHECK (length(trim(name)) > 0),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
) STRICT;

CREATE TABLE roots (
    id           TEXT    NOT NULL PRIMARY KEY,
    library_id   TEXT    NOT NULL REFERENCES libraries (id) ON DELETE CASCADE,
    path         BLOB    NOT NULL,
    status       TEXT    NOT NULL DEFAULT 'available'
                         CHECK (status IN ('available', 'offline')),
    last_seen_at INTEGER,
    created_at   INTEGER NOT NULL,
    UNIQUE (library_id, path)
) STRICT;

CREATE TABLE assets (
    id          TEXT    NOT NULL PRIMARY KEY,
    root_id     TEXT    NOT NULL REFERENCES roots (id) ON DELETE CASCADE,
    rel_path    BLOB    NOT NULL,
    filename    TEXT    NOT NULL,
    ext         TEXT    NOT NULL DEFAULT '',
    kind        TEXT    NOT NULL
                        CHECK (kind IN ('image', 'audio', 'video', 'model3d', 'other')),
    size_bytes  INTEGER NOT NULL CHECK (size_bytes >= 0),
    mtime       INTEGER,
    fs_identity TEXT,
    status      TEXT    NOT NULL DEFAULT 'available'
                        CHECK (status IN ('available', 'missing', 'root_offline', 'unsupported', 'error')),
    is_favorite INTEGER NOT NULL DEFAULT 0 CHECK (is_favorite IN (0, 1)),
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    UNIQUE (root_id, rel_path)
) STRICT;

CREATE INDEX assets_kind ON assets (kind);
CREATE INDEX assets_status ON assets (status);
CREATE INDEX assets_favorite ON assets (is_favorite) WHERE is_favorite = 1;

CREATE TABLE tags (
    id              TEXT    NOT NULL PRIMARY KEY,
    library_id      TEXT    NOT NULL REFERENCES libraries (id) ON DELETE CASCADE,
    name            TEXT    NOT NULL CHECK (length(trim(name)) > 0),
    normalized_name TEXT    NOT NULL,
    color           TEXT,
    created_at      INTEGER NOT NULL,
    UNIQUE (library_id, normalized_name)
) STRICT;

CREATE TABLE asset_tags (
    asset_id TEXT NOT NULL REFERENCES assets (id) ON DELETE CASCADE,
    tag_id   TEXT NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (asset_id, tag_id)
) STRICT, WITHOUT ROWID;

CREATE INDEX asset_tags_tag ON asset_tags (tag_id);

CREATE TABLE collections (
    id          TEXT    NOT NULL PRIMARY KEY,
    library_id  TEXT    NOT NULL REFERENCES libraries (id) ON DELETE CASCADE,
    name        TEXT    NOT NULL CHECK (length(trim(name)) > 0),
    description TEXT,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL
) STRICT;

CREATE TABLE collection_assets (
    collection_id TEXT    NOT NULL REFERENCES collections (id) ON DELETE CASCADE,
    asset_id      TEXT    NOT NULL REFERENCES assets (id) ON DELETE CASCADE,
    added_at      INTEGER NOT NULL,
    PRIMARY KEY (collection_id, asset_id)
) STRICT, WITHOUT ROWID;

CREATE INDEX collection_assets_asset ON collection_assets (asset_id);

-- a tag or collection may only reference assets of its own library
CREATE TRIGGER asset_tags_same_library
BEFORE INSERT ON asset_tags
WHEN (SELECT r.library_id FROM assets a JOIN roots r ON r.id = a.root_id WHERE a.id = NEW.asset_id)
  <> (SELECT library_id FROM tags WHERE id = NEW.tag_id)
BEGIN
    SELECT RAISE(ABORT, 'asset and tag belong to different libraries');
END;

CREATE TRIGGER collection_assets_same_library
BEFORE INSERT ON collection_assets
WHEN (SELECT r.library_id FROM assets a JOIN roots r ON r.id = a.root_id WHERE a.id = NEW.asset_id)
  <> (SELECT library_id FROM collections WHERE id = NEW.collection_id)
BEGIN
    SELECT RAISE(ABORT, 'asset and collection belong to different libraries');
END;
