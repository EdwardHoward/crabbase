-- Your SQL goes here
CREATE TABLE "_collections" (
  id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  schema TEXT NOT NULL DEFAULT '{}'
)