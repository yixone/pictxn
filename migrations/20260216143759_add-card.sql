CREATE TABLE cards (
  id            UUID          NOT NULL  PRIMARY KEY,

  created       TIMESTAMPTZ   NOT NULL,

  title         VARCHAR(255),
  description   TEXT
);
