CREATE TABLE IF NOT EXISTS schema_definitions (
  uuid BLOB PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS schema_properties (
  uuid BLOB PRIMARY KEY NOT NULL,
  schema_definition_id NOT NULL REFERENCES schema_definitions (uuid) ON DELETE CASCADE ON UPDATE CASCADE,
  name TEXT NOT NULL,
  type TEXT NOT NULL,
  UNIQUE(name, schema_definition_id)
);
