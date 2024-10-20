CREATE TABLE IF NOT EXISTS tags (
  uuid BLOB PRIMARY KEY NOT NULL,
  title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tag_parents (
  parent_uuid NOT NULL REFERENCES tags (uuid) ON DELETE CASCADE ON UPDATE CASCADE,
  child_uuid NOT NULL REFERENCES tags (uuid) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY(parent_uuid, child_uuid)
);

CREATE TABLE IF NOT EXISTS note_tags (
  note_uuid NOT NULL REFERENCES notes (uuid) ON DELETE CASCADE ON UPDATE CASCADE,
  tag_uuid NOT NULL REFERENCES tags (uuid) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY(note_uuid, tag_uuid)
);
