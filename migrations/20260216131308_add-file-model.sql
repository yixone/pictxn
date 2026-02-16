CREATE TABLE files (
  id            UUID          NOT NULL  PRIMARY KEY,

  source        UUID          REFERENCES content_sources(id) ON DELETE SET NULL,
  source_url    TEXT,

  created       TIMESTAMPTZ   NOT NULL,

  filename      VARCHAR(255),
  content_type  VARCHAR(64)   NOT NULL  DEFAULT 'application/octet-stream',
  size          BIGINT        NOT NULL  DEFAULT 0
);
