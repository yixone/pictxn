CREATE TABLE cards (
  id            UUID          NOT NULL  PRIMARY KEY,

  file_id       UUID          NOT NULL  REFERENCES files(id) ON DELETE CASCADE,

  created       TIMESTAMPTZ   NOT NULL,

  title         VARCHAR(255),
  description   TEXT
);
