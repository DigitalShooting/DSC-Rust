-- Your SQL goes here

CREATE TABLE session (
  id SERIAL,
  line_id INTEGER,
  data Jsonb,
  PRIMARY KEY (id, line_id)
)
