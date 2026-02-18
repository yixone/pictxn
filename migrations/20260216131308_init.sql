CREATE TABLE files (
  id            UUID          NOT NULL  PRIMARY KEY,

  source        UUID          REFERENCES content_sources(id) ON DELETE SET NULL,
  source_url    TEXT,

  created       TIMESTAMPTZ   NOT NULL,

  filename      VARCHAR(255),
  content_type  VARCHAR(64)   NOT NULL  DEFAULT 'application/octet-stream',
  size          BIGINT        NOT NULL  DEFAULT 0
);

CREATE TABLE content_sources (
  id    UUID          NOT NULL  PRIMARY KEY,

  title VARCHAR(512),

  url   TEXT          NOT NULL  UNIQUE
);

CREATE TABLE cards (
  id            UUID          NOT NULL  PRIMARY KEY,

  file_id       UUID          NOT NULL  REFERENCES files(id) ON DELETE CASCADE,

  created       TIMESTAMPTZ   NOT NULL,

  title         VARCHAR(255),
  description   TEXT
);
