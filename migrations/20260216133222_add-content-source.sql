CREATE TABLE content_sources (
  id    UUID          NOT NULL  PRIMARY KEY,

  title VARCHAR(512),

  url   TEXT          NOT NULL  UNIQUE
);
